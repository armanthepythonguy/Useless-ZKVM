# Useless ZKVM (Zero-Knowledge Virtual Machine)

This repository contains a minimalistic and educational Zero-Knowledge Virtual Machine (ZKVM) implemented using Plonky3. The ZKVM is designed to generate STARK proofs of program execution, providing a simple example for understanding how ZKVMs work.

---

## Table of Contents
1. [Introduction](#introduction)
2. [Architecture Overview](#architecture-overview)
3. [Components](#components)
   - [Virtual Machine (VM)](#virtual-machine-vm)
   - [AIR (Algebraic Intermediate Representation)](#air-algebraic-intermediate-representation)
   - [Trace Generation](#trace-generation)
   - [Prover](#prover)
4. [How It Works](#how-it-works)
5. [Step-by-Step Guide](#step-by-step-guide)
6. [Conclusion](#conclusion)

---

## Introduction
The Useless ZKVM is a toy implementation designed to:
- Execute a stack-based virtual machine program.
- Capture the execution trace for each instruction.
- Generate a STARK proof for the correctness of the execution.

It utilizes Plonky3 libraries to handle the algebraic and cryptographic operations required for proof generation and verification.

---

## Architecture Overview

### 1. **Virtual Machine (VM)**
The VM is a stack-based execution environment. It executes a program consisting of basic arithmetic instructions like:
- `Push`: Push a value onto the stack.
- `Add`: Add the top two stack values.
- `Sub`: Subtract the second stack value from the top.
- `Mul`: Multiply the top two stack values.
- `Div`: Divide the second stack value by the top.

The VM maintains:
- A **stack** (fixed size of 4 elements).
- A **program counter (IP)** to track the current instruction.
- A **trace log** of states for every instruction executed.

### 2. **AIR (Algebraic Intermediate Representation)**
The AIR defines constraints for the VM's execution trace. It ensures that the transitions between states in the trace adhere to the VM's instruction semantics. Each operation (e.g., addition, subtraction) is modeled using polynomial constraints.

### 3. **Trace Generation**
The VM outputs an execution trace, which is a sequence of states. Each state captures:
- The stack values.
- The current instruction being executed.
- Auxiliary data like remainder values for division.

This trace is later extended and padded for compatibility with the STARK proving system.

### 4. **Prover**
The prover:
1. Accepts the execution trace from the VM.
2. Constructs a STARK proof using the Plonky3 library.
3. Verifies the proof by checking that the trace satisfies the AIR constraints.

---

## Components

### Virtual Machine (VM)
- **Core Operations:**
  - Executes stack operations using predefined instructions.
  - Maintains a state log for each executed instruction.

- **Key Methods:**
  - `run`: Executes the program and captures the trace.
  - `perform_operation`: Executes binary operations (e.g., Add, Sub).
  - `get_trace`: Outputs the full execution trace.

### AIR (Algebraic Intermediate Representation)
- Constraints ensure correct state transitions for each instruction type.
- Example constraints:
  - **Addition:** The sum of the top two stack values equals the next stack value.
  - **Push:** Ensures the correct value is pushed onto the stack.

### Trace Generation
- Captures stack values and auxiliary data at each step.
- Pads the trace to ensure compatibility with STARK requirements.

### Prover
- Uses Plonky3’s STARK libraries to generate and verify proofs.
- Components include:
  - **FRI Configuration**: Specifies parameters for the proof.
  - **Challenger**: Handles cryptographic challenges.
  - **PCS (Polynomial Commitment Scheme)**: Verifies constraints efficiently.

---

## How It Works
1. **Program Execution**:
   - A sequence of instructions is executed in the VM.
   - The state of the VM is logged after each instruction.

2. **Trace Construction**:
   - The VM generates a trace capturing each state transition.
   - The trace is extended and padded to fit STARK requirements.

3. **Proof Generation**:
   - The AIR defines constraints that the trace must satisfy.
   - The prover constructs a STARK proof, ensuring the trace adheres to the AIR constraints.

4. **Proof Verification**:
   - The proof is verified using the same AIR constraints, guaranteeing the execution's correctness.

---

## Step-by-Step Guide

1. **Create a Program**:
   - Define a sequence of instructions, e.g.:
     ```rust
     let program = vec![
         Instructions::Push(Mersenne31::from_canonical_u32(10)),
         Instructions::Push(Mersenne31::from_canonical_u32(20)),
         Instructions::Add,
         Instructions::Push(Mersenne31::from_canonical_u32(40)),
         Instructions::Sub,
     ];
     ```

2. **Initialize the VM**:
   - Pass the program to the `VM`:
     ```rust
     let mut vm = VM::new(program);
     ```

3. **Run the VM**:
   - Execute the program and capture the trace:
     ```rust
     vm.run();
     ```

4. **Generate the Proof**:
   - Use the AIR to construct a proof:
     ```rust
     let vmair = VMAir {};
     vmair.generate_proof(vm);
     ```

5. **Verify the Proof**:
   - Verify the proof using the same AIR:
     ```rust
     verify(&config, &vmair, &mut challenger, &proof, &vec![]);
     ```

---

## Conclusion
The Useless ZKVM demonstrates the core principles of zero-knowledge virtual machines:
- Trace construction from program execution.
- Constraint satisfaction using AIR.
- Proof generation and verification using STARKs.

This project is an educational tool, not intended for production use, and serves as a stepping stone for understanding the fundamentals of ZKVMs.

