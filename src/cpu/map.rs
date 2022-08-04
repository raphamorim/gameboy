pub fn mapcb(c: &mut Cpu, m: &mut Mmu) {
    let i=m.r8b(c._r.pc);
    c._r.pc += 1;
    c._r.pc &= 65535;
    // if c._cbmap[i] {
    //     c._cbmap[i]();
    // } else {
    //     alert(i);
    // }
}