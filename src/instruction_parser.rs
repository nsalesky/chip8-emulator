use bitmatch::bitmatch;

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    ClearScreen,
    JumpToMemoryLocation(u16),
    CallSubroutine(u16),
    ReturnFromSubroutine,
    SkipIfRegisterEqValue { vx: u8, value: u8 },
    SkipIfRegisterNeqValue { vx: u8, value: u8 },
    SkipIfRegistersEq { vx: u8, vy: u8 },
    SkipIfRegistersNeq { vx: u8, vy: u8 },
    UpdateRegister { vx: u8, value: u8 },
    AddValueToRegister { vx: u8, value: u8 },
    CopyRegister { vx: u8, vy: u8 },
    BitwiseOR { vx: u8, vy: u8 },
    BitwiseAND { vx: u8, vy: u8 },
    BitwiseXOR { vx: u8, vy: u8 },
    AddRegisterToRegister { vx: u8, vy: u8 },
    SubtractXY { vx: u8, vy: u8 },
    SubtractYX { vx: u8, vy: u8 },
    ShiftLeft { vx: u8, vy: u8 },
    ShiftRight { vx: u8, vy: u8 },
    SetIndexRegister(u16),
    JumpWithOffset(u16),
    GenerateRandomNumber { vx: u8, bitmask: u8 },
    Display { vx: u8, vy: u8, n: u8 },
    SkipIfPressedVX(u8),
    SkipIfNotPressedVX(u8),
    FetchDelayTimerToVX(u8),
    SetDelayTimerToVX(u8),
    SetSoundTimerToVX(u8),
    AddToIndexFromVX(u8),
    WaitForKeyInVX(u8),
    SetIndexToFontCharInVX(u8),
    BinaryCodedDecimalConversionForVX(u8), // FIXME: unclear name
    StoreVariableRegistersToMemoryUpToVX(u8), // FIXME: long awkward name
    LoadMemoryToVariableRegistersFromVXAddress(u8), // FIXME: same here
}

#[bitmatch]
pub fn parse_instruction(instr: u16) -> Option<InstructionType> {
    let (_, x, y, n, nn, nnn) = extract_parts(instr);

    // TODO: could get rid of my own `extract_parts` function and just use the bitmatch variable
    // unpacking
    #[bitmatch]
    match instr {
        "0000_0000_1110_0000" => Some(InstructionType::ClearScreen),
        "0001_????_????_????" => Some(InstructionType::JumpToMemoryLocation(nnn)),
        "0010_????_????_????" => Some(InstructionType::CallSubroutine(nnn)),
        "0000_0000_1110_1110" => Some(InstructionType::ReturnFromSubroutine),
        "0011_????_????_????" => Some(InstructionType::SkipIfRegisterEqValue { vx: x, value: nn }),
        "0100_????_????_????" => Some(InstructionType::SkipIfRegisterNeqValue { vx: x, value: nn }),
        "0101_????_????_0000" => Some(InstructionType::SkipIfRegistersEq { vx: x, vy: y }),
        "1001_????_????_0000" => Some(InstructionType::SkipIfRegistersNeq { vx: x, vy: y }),
        "0110_????_????_????" => Some(InstructionType::UpdateRegister { vx: x, value: nn }),
        "0111_????_????_????" => Some(InstructionType::AddValueToRegister { vx: x, value: nn }),
        "1000_????_????_0000" => Some(InstructionType::CopyRegister { vx: x, vy: y }),
        "1000_????_????_0001" => Some(InstructionType::BitwiseOR { vx: x, vy: y }),
        "1000_????_????_0010" => Some(InstructionType::BitwiseAND { vx: x, vy: y }),
        "1000_????_????_0011" => Some(InstructionType::BitwiseXOR { vx: x, vy: y }),
        "1000_????_????_0100" => Some(InstructionType::AddRegisterToRegister { vx: x, vy: y }),
        "1000_????_????_0101" => Some(InstructionType::SubtractXY { vx: x, vy: y }),
        "1000_????_????_0111" => Some(InstructionType::SubtractYX { vx: x, vy: y }),
        "1000_????_????_0110" => Some(InstructionType::ShiftRight { vx: x, vy: y }),
        "1000_????_????_1110" => Some(InstructionType::ShiftLeft { vx: x, vy: y }),
        "1010_????_????_????" => Some(InstructionType::SetIndexRegister(nnn)),
        "1011_????_????_????" => Some(InstructionType::JumpWithOffset(nnn)),
        "1100_????_????_????" => Some(InstructionType::GenerateRandomNumber { vx: x, bitmask: nn }),
        "1101_????_????_????" => Some(InstructionType::Display { vx: x, vy: y, n }),
        "1110_????_1001_1110" => Some(InstructionType::SkipIfPressedVX(x)),
        "1110_????_1010_0001" => Some(InstructionType::SkipIfNotPressedVX(x)),
        "1111_????_0000_0111" => Some(InstructionType::FetchDelayTimerToVX(x)),
        "1111_????_0001_0101" => Some(InstructionType::SetDelayTimerToVX(x)),
        "1111_????_0001_1000" => Some(InstructionType::SetSoundTimerToVX(x)),
        "1111_????_0001_1110" => Some(InstructionType::AddToIndexFromVX(x)),
        "1111_????_0000_1010" => Some(InstructionType::WaitForKeyInVX(x)),
        "1111_????_0010_1001" => Some(InstructionType::SetIndexToFontCharInVX(x)),
        "1111_????_0011_0011" => Some(InstructionType::BinaryCodedDecimalConversionForVX(x)),
        "1111_????_0101_0101" => Some(InstructionType::StoreVariableRegistersToMemoryUpToVX(x)),
        "1111_????_0110_0101" => Some(InstructionType::LoadMemoryToVariableRegistersFromVXAddress(
            x,
        )),
        _ => None,
    }
}

/// Extract the (`opcode`, `x`, `y`, `n`, `nn`, and `nnn`) components from the given instruction.
fn extract_parts(instr: u16) -> (u8, u8, u8, u8, u8, u16) {
    let opcode = ((0xF000 & instr) >> 12) as u8;
    let x = ((0xF00 & instr) >> 8) as u8;
    let y = ((0xF0 & instr) >> 4) as u8;
    let n = (0xF & instr) as u8;
    let nn = (0xFF & instr) as u8;
    let nnn = 0xFFF & instr;

    (opcode, x, y, n, nn, nnn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0x00E0, Some(InstructionType::ClearScreen))]
    #[case(0x1123, Some(InstructionType::JumpToMemoryLocation(0x123)))]
    #[case(0x00EE, Some(InstructionType::ReturnFromSubroutine))]
    #[case(0x2456, Some(InstructionType::CallSubroutine(0x456)))]
    #[case(0x3AF1, Some(InstructionType::SkipIfRegisterEqValue { vx: 0xA, value: 0xF1 }))]
    #[case(0x4321, Some(InstructionType::SkipIfRegisterNeqValue { vx: 3, value: 0x21 }))]
    #[case(0x5120, Some(InstructionType::SkipIfRegistersEq { vx: 1, vy: 2 }))]
    #[case(0x9F40, Some(InstructionType::SkipIfRegistersNeq { vx: 0xF, vy: 4 }))]
    #[case(0x6234, Some(InstructionType::UpdateRegister { vx: 2, value: 0x34 }))]
    #[case(0x7B00, Some(InstructionType::AddValueToRegister { vx: 0xB, value: 0 }))]
    #[case(0x8380, Some(InstructionType::CopyRegister { vx: 3, vy: 8 }))]
    #[case(0x8501, Some(InstructionType::BitwiseOR { vx: 5, vy: 0 }))]
    #[case(0x84C2, Some(InstructionType::BitwiseAND { vx: 4, vy: 0xC }))]
    #[case(0x8193, Some(InstructionType::BitwiseXOR { vx: 1, vy: 9 }))]
    #[case(0x8024, Some(InstructionType::AddRegisterToRegister { vx: 0, vy: 2 }))]
    #[case(0x8815, Some(InstructionType::SubtractXY { vx: 8, vy: 1 }))]
    #[case(0x8477, Some(InstructionType::SubtractYX { vx: 4, vy: 7 }))]
    #[case(0x8126, Some(InstructionType::ShiftRight { vx: 1, vy: 2 }))]
    #[case(0x8C5E, Some(InstructionType::ShiftLeft { vx: 0xC, vy: 5 }))]
    #[case(0xA987, Some(InstructionType::SetIndexRegister(0x987)))]
    #[case(0xB357, Some(InstructionType::JumpWithOffset(0x357)))]
    #[case(0xC801, Some(InstructionType::GenerateRandomNumber { vx: 8, bitmask: 1 }))]
    #[case(0xD59A, Some(InstructionType::Display { vx: 5, vy: 9, n: 0xA }))]
    #[case(0xE49E, Some(InstructionType::SkipIfPressedVX(4)))]
    #[case(0xE8A1, Some(InstructionType::SkipIfNotPressedVX(8)))]
    #[case(0xF407, Some(InstructionType::FetchDelayTimerToVX(4)))]
    #[case(0xFE15, Some(InstructionType::SetDelayTimerToVX(0xE)))]
    #[case(0xF018, Some(InstructionType::SetSoundTimerToVX(0)))]
    #[case(0xFF1E, Some(InstructionType::AddToIndexFromVX(0xF)))]
    #[case(0xF20A, Some(InstructionType::WaitForKeyInVX(2)))]
    #[case(0xF729, Some(InstructionType::SetIndexToFontCharInVX(7)))]
    #[case(0xFD33, Some(InstructionType::BinaryCodedDecimalConversionForVX(0xD)))]
    #[case(0xF455, Some(InstructionType::StoreVariableRegistersToMemoryUpToVX(4)))]
    #[case(
        0xFA65,
        Some(InstructionType::LoadMemoryToVariableRegistersFromVXAddress(0xA))
    )]
    #[case(0xF008, None)]
    fn parse_instruction_test(#[case] input: u16, #[case] expected: Option<InstructionType>) {
        assert_eq!(parse_instruction(input), expected);
    }

    #[test]
    fn extract_parts_works() {
        let (opcode, x, y, n, nn, nnn) = extract_parts(0x39A0);

        assert_eq!(opcode, 0x3);
        assert_eq!(x, 0x9);
        assert_eq!(y, 0xA);
        assert_eq!(n, 0);
        assert_eq!(nn, 0xA0);
        assert_eq!(nnn, 0x9A0);
    }
}
