use std::ops::BitAnd;

/** \brief PIO instruction encoding
 *  \defgroup pio_instructions pio_instructions
 *  \ingroup hardware_pio
 *
 * Functions for generating PIO instruction encodings programmatically. In debug builds
 *  `PARAM_ASSERTIONS_ENABLED_PIO_INSTRUCTIONS` can be set to 1 to enable validation of encoding function
* parameters.
*
* For fuller descriptions of the instructions in question see the "RP2040 Datasheet"
*/
// PICO_CONFIG: PARAM_ASSERTIONS_ENABLED_PIO_INSTRUCTIONS, Enable/disable assertions in the PIO instructions, type=bool, default=0, group=pio_instructions
pub const PARAM_ASSERTIONS_ENABLED_PIO_INSTRUCTIONS: bool =
    cfg!(feature = "PARAM_ASSERTIONS_ENABLED_PIO_INSTRUCTIONS");

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum pio_instr_bits {
    pio_instr_bits_jmp = 0x0000,
    pio_instr_bits_wait = 0x2000,
    pio_instr_bits_in = 0x4000,
    pio_instr_bits_out = 0x6000,
    pio_instr_bits_push = 0x8000,
    pio_instr_bits_pull = 0x8080,
    pio_instr_bits_mov = 0xa000,
    pio_instr_bits_irq = 0xc000,
    pio_instr_bits_set = 0xe000,
}

#[cfg(not(feature = "NDEBUG"))] pub const _PIO_INVALID_IN_SRC: u32 = 0x08;
#[cfg(not(feature = "NDEBUG"))] pub const _PIO_INVALID_OUT_DEST: u32 = 0x10;
#[cfg(not(feature = "NDEBUG"))] pub const _PIO_INVALID_SET_DEST: u32 = 0x20;
#[cfg(not(feature = "NDEBUG"))] pub const _PIO_INVALID_MOV_SRC: u32 = 0x40;
#[cfg(not(feature = "NDEBUG"))] pub const _PIO_INVALID_MOV_DEST: u32 = 0x80;
#[cfg(feature = "NDEBUG")]      pub const _PIO_INVALID_IN_SRC: u32 = 0;
#[cfg(feature = "NDEBUG")]      pub const _PIO_INVALID_OUT_DEST: u32 = 0;
#[cfg(feature = "NDEBUG")]      pub const _PIO_INVALID_SET_DEST: u32 = 0;
#[cfg(feature = "NDEBUG")]      pub const _PIO_INVALID_MOV_SRC: u32 = 0;
#[cfg(feature = "NDEBUG")]      pub const _PIO_INVALID_MOV_DEST: u32 = 0;

/* \brief Enumeration of values to pass for source/destination args for instruction encoding functions
*  \ingroup pio_instructions
*
* \note Not all values are suitable for all functions. Validity is only checked in debug mode when
* `PARAM_ASSERTIONS_ENABLED_PIO_INSTRUCTIONS` is 1
*/
#[allow(non_camel_case_types)]
pub struct pio_src_dest(pub u32);
impl pio_src_dest {
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_pins    : pio_src_dest = pio_src_dest(0);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_x       : pio_src_dest = pio_src_dest(1);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_y       : pio_src_dest = pio_src_dest(2);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_null    : pio_src_dest = pio_src_dest(3 | _PIO_INVALID_SET_DEST | _PIO_INVALID_MOV_DEST);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_pindirs : pio_src_dest = pio_src_dest(4 | _PIO_INVALID_IN_SRC | _PIO_INVALID_MOV_SRC | _PIO_INVALID_MOV_DEST);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_exec_mov: pio_src_dest = pio_src_dest(4 | _PIO_INVALID_IN_SRC | _PIO_INVALID_OUT_DEST | _PIO_INVALID_SET_DEST | _PIO_INVALID_MOV_SRC);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_status  : pio_src_dest = pio_src_dest(5 | _PIO_INVALID_IN_SRC | _PIO_INVALID_OUT_DEST | _PIO_INVALID_SET_DEST | _PIO_INVALID_MOV_DEST);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_pc      : pio_src_dest = pio_src_dest(5 | _PIO_INVALID_IN_SRC | _PIO_INVALID_SET_DEST | _PIO_INVALID_MOV_SRC);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_isr     : pio_src_dest = pio_src_dest(6 | _PIO_INVALID_SET_DEST);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_osr     : pio_src_dest = pio_src_dest(7 | _PIO_INVALID_OUT_DEST | _PIO_INVALID_SET_DEST);
    #[allow(non_camel_case_types, non_upper_case_globals)] pub const pio_exec_out: pio_src_dest = pio_src_dest(7 | _PIO_INVALID_IN_SRC | _PIO_INVALID_SET_DEST | _PIO_INVALID_MOV_SRC | _PIO_INVALID_MOV_DEST);
}
impl BitAnd<u32> for pio_src_dest {
    type Output = u32;
    fn bitand(self, rhs: u32) -> Self::Output {
        self.0 & rhs
    }
}
impl BitAnd<u32> for &pio_src_dest {
    type Output = u32;
    fn bitand(self, rhs: u32) -> Self::Output {
        self.0 & rhs
    }
}

pub fn _pio_major_instr_bits(instr: u32) -> u32 {
    instr & 0xe000
}

pub fn _pio_encode_instr_and_args(instr_bits: pio_instr_bits, arg1: u32, arg2: u32) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, arg1 <= 0x7);
    let instr_bits = instr_bits as u32;
    if crate::PARAM_ASSERTIONS_ENABLED!(PIO_INSTRUCTIONS) {
        let major = _pio_major_instr_bits(instr_bits);
        if major == pio_instr_bits::pio_instr_bits_in as u32
            || major == pio_instr_bits::pio_instr_bits_out as u32
        {
            assert!(arg2 != 0 && arg2 <= 32);
        } else {
            assert!(arg2 <= 31);
        }
    }
    instr_bits | (arg1 << 5) | (arg2 & 0x1f)
}

pub fn _pio_encode_instr_and_src_dest(
    instr_bits: pio_instr_bits,
    dest: pio_src_dest,
    value: u32,
) -> u32 {
    _pio_encode_instr_and_args(instr_bits, dest & 7, value)
}

/* \brief Encode just the delay slot bits of an instruction
*  \ingroup pio_instructions
*
* \note This function does not return a valid instruction encoding; instead it returns an encoding of the delay
* slot suitable for `OR`ing with the result of an encoding function for an actual instruction. Care should be taken when
* combining the results of this function with the results of \ref pio_encode_sideset and \ref pio_encode_sideset_opt
* as they share the same bits within the instruction encoding.
*
* \param cycles the number of cycles 0-31 (or less if side set is being used)
* \return the delay slot bits to be ORed with an instruction encoding
*/
pub fn pio_encode_delay(cycles: u32) -> u32 {
    // note that the maximum cycles will be smaller if sideset_bit_count > 0
    crate::valid_params_if!(PIO_INSTRUCTIONS, cycles <= 0x1f);
    cycles << 8
}

/* \brief Encode just the side set bits of an instruction (in non optional side set mode)
*  \ingroup pio_instructions
*
* \note This function does not return a valid instruction encoding; instead it returns an encoding of the side set bits
* suitable for `OR`ing with the result of an encoding function for an actual instruction. Care should be taken when
* combining the results of this function with the results of \ref pio_encode_delay as they share the same bits
* within the instruction encoding.
*
* \param sideset_bit_count number of side set bits as would be specified via `.sideset` in pioasm
* \param value the value to sideset on the pins
* \return the side set bits to be ORed with an instruction encoding
*/
pub fn pio_encode_sideset(sideset_bit_count: u32, value: u32) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, (1..=5).contains(&sideset_bit_count));
    crate::valid_params_if!(PIO_INSTRUCTIONS, value <= ((1 << sideset_bit_count) - 1));
    value << (13 - sideset_bit_count)
}

/* \brief Encode just the side set bits of an instruction (in optional -`opt` side set mode)
*  \ingroup pio_instructions
*
* \note This function does not return a valid instruction encoding; instead it returns an encoding of the side set bits
* suitable for `OR`ing with the result of an encoding function for an actual instruction. Care should be taken when
* combining the results of this function with the results of \ref pio_encode_delay as they share the same bits
* within the instruction encoding.
*
* \param sideset_bit_count number of side set bits as would be specified via `.sideset <n> opt` in pioasm
* \param value the value to sideset on the pins
* \return the side set bits to be ORed with an instruction encoding
*/
pub fn pio_encode_sideset_opt(sideset_bit_count: u32, value: u32) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, (1..=4).contains(&sideset_bit_count));
    crate::valid_params_if!(PIO_INSTRUCTIONS, value <= ((1 << sideset_bit_count) - 1));
    0x1000 | value << (12 - sideset_bit_count)
}

/* \brief Encode an unconditional JMP instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `JMP <addr>`
*
* \param addr The target address 0-31 (an absolute address within the PIO instruction memory)
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_jmp(addr: u32) -> u32 {
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_jmp, 0, addr)
}

/* \brief Encode a conditional JMP if scratch X zero instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `JMP !X <addr>`
*
* \param addr The target address 0-31 (an absolute address within the PIO instruction memory)
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_jmp_not_x(addr: u32) -> u32 {
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_jmp, 1, addr)
}

/* \brief Encode a conditional JMP if scratch X non-zero (and post-decrement X) instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `JMP X-- <addr>`
*
* \param addr The target address 0-31 (an absolute address within the PIO instruction memory)
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_jmp_x_dec(addr: u32) -> u32 {
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_jmp, 2, addr)
}

/* \brief Encode a conditional JMP if scratch Y zero instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `JMP !Y <addr>`
*
* \param addr The target address 0-31 (an absolute address within the PIO instruction memory)
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_jmp_not_y(addr: u32) -> u32 {
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_jmp, 3, addr)
}

/* \brief Encode a conditional JMP if scratch Y non-zero (and post-decrement Y) instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `JMP Y-- <addr>`
*
* \param addr The target address 0-31 (an absolute address within the PIO instruction memory)
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_jmp_y_dec(addr: u32) -> u32 {
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_jmp, 4, addr)
}

/* \brief Encode a conditional JMP if scratch X not equal scratch Y instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `JMP X!=Y <addr>`
*
* \param addr The target address 0-31 (an absolute address within the PIO instruction memory)
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_jmp_x_ne_y(addr: u32) -> u32 {
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_jmp, 5, addr)
}

/* \brief Encode a conditional JMP if input pin high instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `JMP PIN <addr>`
*
* \param addr The target address 0-31 (an absolute address within the PIO instruction memory)
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_jmp_pin(addr: u32) -> u32 {
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_jmp, 6, addr)
}

/* \brief Encode a conditional JMP if output shift register not empty instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `JMP !OSRE <addr>`
*
* \param addr The target address 0-31 (an absolute address within the PIO instruction memory)
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_jmp_not_osre(addr: u32) -> u32 {
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_jmp, 7, addr)
}

pub fn _pio_encode_irq(relative: bool, irq: u32) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, irq <= 7);
    if relative {
        0x10 | irq
    } else {
        irq
    }
}

/* \brief Encode a WAIT for GPIO pin instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `WAIT <polarity> GPIO <gpio>`
*
* \param polarity true for `WAIT 1`, false for `WAIT 0`
* \param gpio The real GPIO number 0-31
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_wait_gpio(polarity: bool, gpio: u32) -> u32 {
    let arg1 = if polarity { 4 } else { 0 };
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_wait, arg1, gpio)
}

/* \brief Encode a WAIT for pin instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `WAIT <polarity> PIN <pin>`
*
* \param polarity true for `WAIT 1`, false for `WAIT 0`
* \param pin The pin number 0-31 relative to the executing SM's input pin mapping
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_wait_pin(polarity: bool, pin: u32) -> u32 {
    let arg1 = 1 | if polarity { 4 } else { 0 };
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_wait, arg1, pin)
}

/* \brief Encode a WAIT for IRQ instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `WAIT <polarity> IRQ <irq> <relative>`
*
* \param polarity true for `WAIT 1`, false for `WAIT 0`
* \param relative true for a `WAIT IRQ <irq> REL`, false for regular `WAIT IRQ <irq>`
* \param irq the irq number 0-7
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_wait_irq(polarity: bool, relative: bool, irq: u32) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, irq <= 7);
    let arg1 = 2 | if polarity { 4 } else { 0 };
    _pio_encode_instr_and_args(
        pio_instr_bits::pio_instr_bits_wait,
        arg1,
        _pio_encode_irq(relative, irq),
    )
}

/* \brief Encode an IN instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `IN <src>, <count>`
*
* \param src The source to take data from
* \param count The number of bits 1-32
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
#[allow(clippy::bad_bit_mask)]
pub fn pio_encode_in(src: pio_src_dest, count: u32) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&src & _PIO_INVALID_IN_SRC));
    _pio_encode_instr_and_src_dest(pio_instr_bits::pio_instr_bits_in, src, count)
}

/* \brief Encode an OUT instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `OUT <src>, <count>`
*
* \param dest The destination to write data to
* \param count The number of bits 1-32
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
#[allow(clippy::bad_bit_mask)]
pub fn pio_encode_out(dest: pio_src_dest, count: u32) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&dest & _PIO_INVALID_OUT_DEST));
    _pio_encode_instr_and_src_dest(pio_instr_bits::pio_instr_bits_out, dest, count)
}

/* \brief Encode a PUSH instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `PUSH <if_full>, <block>`
*
* \param if_full true for `PUSH IF_FULL ...`, false for `PUSH ...`
* \param block true for `PUSH ... BLOCK`, false for `PUSH ...`
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_push(if_full: bool, block: bool) -> u32 {
    let arg1_p1 = if if_full { 2 } else { 0 };
    let arg1_p2 = if block { 1 } else { 0 };
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_push, arg1_p1 | arg1_p2, 0)
}

/* \brief Encode a PULL instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `PULL <if_empty>, <block>`
*
* \param if_empty true for `PULL IF_EMPTY ...`, false for `PULL ...`
* \param block true for `PULL ... BLOCK`, false for `PULL ...`
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_pull(if_empty: bool, block: bool) -> u32 {
    let arg1_p1 = if if_empty { 2 } else { 0 };
    let arg1_p2 = if block { 1 } else { 0 };
    _pio_encode_instr_and_args(pio_instr_bits::pio_instr_bits_pull, arg1_p1 | arg1_p2, 0)
}

/* \brief Encode a MOV instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `MOV <dest>, <src>`
*
* \param dest The destination to write data to
* \param src The source to take data from
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
#[allow(clippy::bad_bit_mask)]
pub fn pio_encode_mov(dest: pio_src_dest, src: pio_src_dest) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&dest & _PIO_INVALID_MOV_DEST));
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&src & _PIO_INVALID_MOV_SRC));
    _pio_encode_instr_and_src_dest(pio_instr_bits::pio_instr_bits_mov, dest, src & 7)
}

/* \brief Encode a MOV instruction with bit invert
*  \ingroup pio_instructions
*
* This is the equivalent of `MOV <dest>, ~<src>`
*
* \param dest The destination to write inverted data to
* \param src The source to take data from
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
#[allow(clippy::bad_bit_mask)]
pub fn pio_encode_mov_not(dest: pio_src_dest, src: pio_src_dest) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&dest & _PIO_INVALID_MOV_DEST));
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&src & _PIO_INVALID_MOV_SRC));
    _pio_encode_instr_and_src_dest(
        pio_instr_bits::pio_instr_bits_mov,
        dest,
        (1 << 3) | (src & 7),
    )
}

/* \brief Encode a MOV instruction with bit reverse
*  \ingroup pio_instructions
*
* This is the equivalent of `MOV <dest>, ::<src>`
*
* \param dest The destination to write bit reversed data to
* \param src The source to take data from
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
#[allow(clippy::bad_bit_mask)]
pub fn pio_encode_mov_reverse(dest: pio_src_dest, src: pio_src_dest) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&dest & _PIO_INVALID_MOV_DEST));
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&src & _PIO_INVALID_MOV_SRC));
    _pio_encode_instr_and_src_dest(
        pio_instr_bits::pio_instr_bits_mov,
        dest,
        (2 << 3) | (src & 7),
    )
}

/* \brief Encode a IRQ SET instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `IRQ SET <irq> <relative>`
*
* \param relative true for a `IRQ SET <irq> REL`, false for regular `IRQ SET <irq>`
* \param irq the irq number 0-7
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_irq_set(relative: bool, irq: u32) -> u32 {
    _pio_encode_instr_and_args(
        pio_instr_bits::pio_instr_bits_irq,
        0,
        _pio_encode_irq(relative, irq),
    )
}

/* \brief Encode a IRQ WAIT instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `IRQ WAIT <irq> <relative>`
*
* \param relative true for a `IRQ WAIT <irq> REL`, false for regular `IRQ WAIT <irq>`
* \param irq the irq number 0-7
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_irq_wait(relative: bool, irq: u32) -> u32 {
    _pio_encode_instr_and_args(
        pio_instr_bits::pio_instr_bits_irq,
        1,
        _pio_encode_irq(relative, irq),
    )
}

/* \brief Encode a IRQ CLEAR instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `IRQ CLEAR <irq> <relative>`
*
* \param relative true for a `IRQ CLEAR <irq> REL`, false for regular `IRQ CLEAR <irq>`
* \param irq the irq number 0-7
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_irq_clear(relative: bool, irq: u32) -> u32 {
    _pio_encode_instr_and_args(
        pio_instr_bits::pio_instr_bits_irq,
        2,
        _pio_encode_irq(relative, irq),
    )
}

/* \brief Encode a SET instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `SET <dest>, <value>`
*
* \param dest The destination to apply the value to
* \param value The value 0-31
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
#[allow(clippy::bad_bit_mask)]
pub fn pio_encode_set(dest: pio_src_dest, value: u32) -> u32 {
    crate::valid_params_if!(PIO_INSTRUCTIONS, 0 == (&dest & _PIO_INVALID_SET_DEST));
    _pio_encode_instr_and_src_dest(pio_instr_bits::pio_instr_bits_set, dest, value)
}

/* \brief Encode a NOP instruction
*  \ingroup pio_instructions
*
* This is the equivalent of `NOP` which is itself encoded as `MOV y, y`
*
* \return The instruction encoding with 0 delay and no side set value
* \see pio_encode_delay, pio_encode_sideset, pio_encode_sideset_opt
*/
pub fn pio_encode_nop() -> u32 {
    pio_encode_mov(pio_src_dest::pio_y, pio_src_dest::pio_y)
}
