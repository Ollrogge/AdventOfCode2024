use ast::BV;
use num_enum::FromPrimitive;
use regex::Regex;
use z3::ast::Ast;
use z3::*;

#[derive(Debug, Clone)]
struct Cpu {
    regs: [u64; 3],
    pc: usize,
}

#[repr(u64)]
#[derive(Debug, FromPrimitive)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
    #[default]
    Nop,
}

fn combo_operand_val(cpu: &Cpu, operand: u64) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => cpu.regs[0],
        5 => cpu.regs[1],
        6 => cpu.regs[2],
        _ => panic!("Unexpected operand: {}", operand),
    }
}

fn parse_input(input: &str) -> (Cpu, Vec<u64>) {
    let re = Regex::new(r"\d+").unwrap();
    let s = input.split("\n\n").collect::<Vec<&str>>();

    let mut cpu = Cpu {
        regs: [0; 3],
        pc: 0,
    };
    for (i, l) in s[0].lines().enumerate() {
        cpu.regs[i] = re
            .find(l)
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .unwrap();
    }

    let program: Vec<u64> = re
        .find_iter(s[1].lines().nth(0).unwrap())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    (cpu, program)
}

fn program_loop(cpu: &mut Cpu, program: &Vec<u64>) -> Vec<u64> {
    let mut output = Vec::new();
    while cpu.pc < program.len() {
        match Instruction::from_primitive(program[cpu.pc]) {
            Instruction::Adv => {
                cpu.regs[0] /= 1 << combo_operand_val(&cpu, program[cpu.pc + 1]);

                cpu.pc += 2;
            }
            Instruction::Bxl => {
                cpu.regs[1] ^= program[cpu.pc + 1];
                cpu.pc += 2;
            }
            Instruction::Bst => {
                cpu.regs[1] = combo_operand_val(&cpu, program[cpu.pc + 1]) % 8;
                cpu.pc += 2;
            }
            Instruction::Jnz => {
                if cpu.regs[0] == 0 {
                    cpu.pc += 2;
                } else {
                    cpu.pc = program[cpu.pc + 1] as usize;
                }
            }
            Instruction::Bxc => {
                cpu.regs[1] ^= cpu.regs[2];
                cpu.pc += 2;
            }
            Instruction::Out => {
                let val = combo_operand_val(&cpu, program[cpu.pc + 1]) % 8;
                output.push(val);

                cpu.pc += 2;
            }
            Instruction::Bdv => {
                cpu.regs[1] = cpu.regs[0] / (1 << combo_operand_val(&cpu, program[cpu.pc + 1]));

                cpu.pc += 2;
            }
            Instruction::Cdv => {
                cpu.regs[2] = cpu.regs[0] / (1 << combo_operand_val(&cpu, program[cpu.pc + 1]));

                cpu.pc += 2;
            }
            Instruction::Nop => {
                panic!("Unexpected nop instruction")
            }
        }
    }

    output
}

fn part1(input: &str) {
    let (mut cpu, program) = parse_input(input);

    let output = program_loop(&mut cpu, &program);

    println!(
        "{}",
        output
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn part2(input: &str) {
    let (mut cpu, program) = parse_input(input);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let A = BV::new_const(&ctx, "A", 64);
    let mut B = BV::from_u64(&ctx, 0, 64); // Start B as 0
    let mut C = BV::from_u64(&ctx, 0, 64); // Start C as 0

    let mut A_var = A.clone(); // Track current state of A

    let mut ip = 0; // Instruction pointer starts at 0
    let mut idx = 0x0;
    while idx != program.len() {
        let operand = program[ip + 1];

        match Instruction::from_primitive(program[ip]) {
            Instruction::Adv => {
                if operand < 4 {
                    let divisor = BV::from_u64(&ctx, 1, 64).bvshl(&BV::from_u64(&ctx, operand, 64));
                    A_var = A_var.bvudiv(&divisor);
                } else {
                    let operand = match operand {
                        4 => &A_var,
                        5 => &B,
                        6 => &C,
                        _ => panic!("Unexpected operand"),
                    };
                    let divisor = BV::from_u64(&ctx, 2, 64).bvshl(operand);
                    A_var = A_var.bvudiv(&divisor);
                }
            }
            Instruction::Bxl => {
                B = B.bvxor(&BV::from_u64(&ctx, operand as u64, 64));
            }
            Instruction::Bst => {
                if operand < 4 {
                    B = BV::from_u64(&ctx, operand, 64).bvurem(&BV::from_u64(&ctx, 8, 64));
                } else {
                    let operand = match operand {
                        4 => &A_var,
                        5 => &B,
                        6 => &C,
                        _ => panic!("Unexpected operand"),
                    };
                    B = operand.bvurem(&BV::from_u64(&ctx, 8, 64));
                }
            }
            Instruction::Jnz => {
                if solver.check_assumptions(&[A_var._eq(&BV::from_u64(&ctx, 0, 64)).not()])
                    == SatResult::Sat
                {
                    ip = operand as usize;
                    continue;
                }
            }
            Instruction::Bxc => {
                B = B.bvxor(&C);
            }
            Instruction::Out => {
                if operand < 4 {
                    let val = BV::from_u64(&ctx, operand, 64);
                    solver.assert(&val.bvurem(&BV::from_u64(&ctx, 8, 64))._eq(&BV::from_u64(
                        &ctx,
                        program[idx],
                        64,
                    )));
                } else {
                    let val = match operand {
                        4 => &A_var,
                        5 => &B,
                        6 => &C,
                        _ => panic!("Unexpected operand"),
                    };
                    solver.assert(&val.bvurem(&BV::from_u64(&ctx, 8, 64))._eq(&BV::from_u64(
                        &ctx,
                        program[idx],
                        64,
                    )));
                }

                idx += 1;
            }
            Instruction::Bdv => {
                if operand < 4 {
                    let divisor = BV::from_u64(&ctx, 1, 64).bvshl(&BV::from_u64(&ctx, operand, 64));
                    B = A_var.bvudiv(&divisor);
                } else {
                    let operand = match operand {
                        4 => &A_var,
                        5 => &B,
                        6 => &C,
                        _ => panic!("Unexpected operand"),
                    };
                    let divisor = BV::from_u64(&ctx, 1, 64).bvshl(operand);
                    B = A_var.bvudiv(&divisor);
                }
            }
            Instruction::Cdv => {
                if operand < 4 {
                    let divisor = BV::from_u64(&ctx, 1, 64).bvshl(&BV::from_u64(&ctx, operand, 64));
                    C = A_var.bvudiv(&divisor);
                } else {
                    let operand = match operand {
                        4 => &A_var,
                        5 => &B,
                        6 => &C,
                        _ => panic!("Unexpected operand"),
                    };
                    let divisor = BV::from_u64(&ctx, 1, 64).bvshl(operand);
                    C = A_var.bvudiv(&divisor);
                }
            }
            _ => panic!("Unknown opcode!"),
        }

        ip += 2; // Move to the next instruction
    }

    solver.assert(&A_var._eq(&BV::from_u64(&ctx, 0, 64)));

    // Solve
    match solver.check() {
        SatResult::Sat => {
            println!("SAT: Solution found!");
            let model = solver.get_model().unwrap();
            let A_res = model.eval(&A, true).unwrap();
            let A_res = A_res.as_u64().expect("Failed to convert BV to u64");

            println!("Initial A: = {}", A_res);

            cpu.regs[0] = A_res;

            let res = program_loop(&mut cpu, &program);
            assert_eq!(res, program);
        }
        SatResult::Unsat => println!("UNSAT: No solution exists."),
        SatResult::Unknown => println!("UNKNOWN: Solver could not determine satisfiability."),
    }
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
