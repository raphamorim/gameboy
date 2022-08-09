// http://imrannazar.com/content/files/jsgb.z80.js

// use crate::cpu::cpu::Cpu;

// fn LDrr_bb(c: &mut Cpu) { c._r.b=c._r.b; c._r.m=1; c._r.t=4; }
// fn LDrr_bc(c: &mut Cpu) { c._r.b=c._r.c; c._r.m=1; c._r.t=4; }
// fn LDrr_bd(c: &mut Cpu) { c._r.b=c._r.d; c._r.m=1; c._r.t=4; }
// fn LDrr_be(c: &mut Cpu) { c._r.b=c._r.e; c._r.m=1; c._r.t=4; }
// fn LDrr_bh(c: &mut Cpu) { c._r.b=c._r.h; c._r.m=1; c._r.t=4; }
// fn LDrr_bl(c: &mut Cpu) { c._r.b=c._r.l; c._r.m=1; c._r.t=4; }
// fn LDrr_ba(c: &mut Cpu) { c._r.b=c._r.a; c._r.m=1; c._r.t=4; }
// fn LDrr_cb(c: &mut Cpu) { c._r.c=c._r.b; c._r.m=1; c._r.t=4; }
// fn LDrr_cc(c: &mut Cpu) { c._r.c=c._r.c; c._r.m=1; c._r.t=4; }
// fn LDrr_cd(c: &mut Cpu) { c._r.c=c._r.d; c._r.m=1; c._r.t=4; }
// fn LDrr_ce(c: &mut Cpu) { c._r.c=c._r.e; c._r.m=1; c._r.t=4; }
// fn LDrr_ch(c: &mut Cpu) { c._r.c=c._r.h; c._r.m=1; c._r.t=4; }
// fn LDrr_cl(c: &mut Cpu) { c._r.c=c._r.l; c._r.m=1; c._r.t=4; }
// fn LDrr_ca(c: &mut Cpu) { c._r.c=c._r.a; c._r.m=1; c._r.t=4; }
// fn LDrr_db(c: &mut Cpu) { c._r.d=c._r.b; c._r.m=1; c._r.t=4; }
// fn LDrr_dc(c: &mut Cpu) { c._r.d=c._r.c; c._r.m=1; c._r.t=4; }
// fn LDrr_dd(c: &mut Cpu) { c._r.d=c._r.d; c._r.m=1; c._r.t=4; }
// fn LDrr_de(c: &mut Cpu) { c._r.d=c._r.e; c._r.m=1; c._r.t=4; }
// fn LDrr_dh(c: &mut Cpu) { c._r.d=c._r.h; c._r.m=1; c._r.t=4; }
// fn LDrr_dl(c: &mut Cpu) { c._r.d=c._r.l; c._r.m=1; c._r.t=4; }
// fn LDrr_da(c: &mut Cpu) { c._r.d=c._r.a; c._r.m=1; c._r.t=4; }
// fn LDrr_eb(c: &mut Cpu) { c._r.e=c._r.b; c._r.m=1; c._r.t=4; }
// fn LDrr_ec(c: &mut Cpu) { c._r.e=c._r.c; c._r.m=1; c._r.t=4; }
// fn LDrr_ed(c: &mut Cpu) { c._r.e=c._r.d; c._r.m=1; c._r.t=4; }
// fn LDrr_ee(c: &mut Cpu) { c._r.e=c._r.e; c._r.m=1; c._r.t=4; }
// fn LDrr_eh(c: &mut Cpu) { c._r.e=c._r.h; c._r.m=1; c._r.t=4; }
// fn LDrr_el(c: &mut Cpu) { c._r.e=c._r.l; c._r.m=1; c._r.t=4; }
// fn LDrr_ea(c: &mut Cpu) { c._r.e=c._r.a; c._r.m=1; c._r.t=4; }
// fn LDrr_hb(c: &mut Cpu) { c._r.h=c._r.b; c._r.m=1; c._r.t=4; }
// fn LDrr_hc(c: &mut Cpu) { c._r.h=c._r.c; c._r.m=1; c._r.t=4; }
// fn LDrr_hd(c: &mut Cpu) { c._r.h=c._r.d; c._r.m=1; c._r.t=4; }
// fn LDrr_he(c: &mut Cpu) { c._r.h=c._r.e; c._r.m=1; c._r.t=4; }
// fn LDrr_hh(c: &mut Cpu) { c._r.h=c._r.h; c._r.m=1; c._r.t=4; }
// fn LDrr_hl(c: &mut Cpu) { c._r.h=c._r.l; c._r.m=1; c._r.t=4; }
// fn LDrr_ha(c: &mut Cpu) { c._r.h=c._r.a; c._r.m=1; c._r.t=4; }
// fn LDrr_lb(c: &mut Cpu) { c._r.l=c._r.b; c._r.m=1; c._r.t=4; }
// fn LDrr_lc(c: &mut Cpu) { c._r.l=c._r.c; c._r.m=1; c._r.t=4; }
// fn LDrr_ld(c: &mut Cpu) { c._r.l=c._r.d; c._r.m=1; c._r.t=4; }
// fn LDrr_le(c: &mut Cpu) { c._r.l=c._r.e; c._r.m=1; c._r.t=4; }
// fn LDrr_lh(c: &mut Cpu) { c._r.l=c._r.h; c._r.m=1; c._r.t=4; }
// fn LDrr_ll(c: &mut Cpu) { c._r.l=c._r.l; c._r.m=1; c._r.t=4; }
// fn LDrr_la(c: &mut Cpu) { c._r.l=c._r.a; c._r.m=1; c._r.t=4; }
// fn LDrr_ab(c: &mut Cpu) { c._r.a=c._r.b; c._r.m=1; c._r.t=4; }
// fn LDrr_ac(c: &mut Cpu) { c._r.a=c._r.c; c._r.m=1; c._r.t=4; }
// fn LDrr_ad(c: &mut Cpu) { c._r.a=c._r.d; c._r.m=1; c._r.t=4; }
// fn LDrr_ae(c: &mut Cpu) { c._r.a=c._r.e; c._r.m=1; c._r.t=4; }
// fn LDrr_ah(c: &mut Cpu) { c._r.a=c._r.h; c._r.m=1; c._r.t=4; }
// fn LDrr_al(c: &mut Cpu) { c._r.a=c._r.l; c._r.m=1; c._r.t=4; }
// fn LDrr_aa(c: &mut Cpu) { c._r.a=c._r.a; c._r.m=1; c._r.t=4; }

// fn LDrHLm_b(c: &mut Cpu) { c._r.b=MMU.rb((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
// fn LDrHLm_c(c: &mut Cpu) { c._r.c=MMU.rb((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
// fn LDrHLm_d(c: &mut Cpu) { c._r.d=MMU.rb((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
// fn LDrHLm_e(c: &mut Cpu) { c._r.e=MMU.rb((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
// fn LDrHLm_h(c: &mut Cpu) { c._r.h=MMU.rb((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
// fn LDrHLm_l(c: &mut Cpu) { c._r.l=MMU.rb((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
// fn LDrHLm_a(c: &mut Cpu) { c._r.a=MMU.rb((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }

// fn LDHLmr_b(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l,c._r.b); c._r.m=2; c._r.t=8; }
// fn LDHLmr_c(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l,c._r.c); c._r.m=2; c._r.t=8; }
// fn LDHLmr_d(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l,c._r.d); c._r.m=2; c._r.t=8; }
// fn LDHLmr_e(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l,c._r.e); c._r.m=2; c._r.t=8; }
// fn LDHLmr_h(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l,c._r.h); c._r.m=2; c._r.t=8; }
// fn LDHLmr_l(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l,c._r.l); c._r.m=2; c._r.t=8; }
// fn LDHLmr_a(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l,c._r.a); c._r.m=2; c._r.t=8; }

// fn LDrn_b(c: &mut Cpu) { c._r.b=MMU.rb(c._r.pc); c._r.pc++; c._r.m=2; c._r.t=8; }
// fn LDrn_c(c: &mut Cpu) { c._r.c=MMU.rb(c._r.pc); c._r.pc++; c._r.m=2; c._r.t=8; }
// fn LDrn_d(c: &mut Cpu) { c._r.d=MMU.rb(c._r.pc); c._r.pc++; c._r.m=2; c._r.t=8; }
// fn LDrn_e(c: &mut Cpu) { c._r.e=MMU.rb(c._r.pc); c._r.pc++; c._r.m=2; c._r.t=8; }
// fn LDrn_h(c: &mut Cpu) { c._r.h=MMU.rb(c._r.pc); c._r.pc++; c._r.m=2; c._r.t=8; }
// fn LDrn_l(c: &mut Cpu) { c._r.l=MMU.rb(c._r.pc); c._r.pc++; c._r.m=2; c._r.t=8; }
// fn LDrn_a(c: &mut Cpu) { c._r.a=MMU.rb(c._r.pc); c._r.pc++; c._r.m=2; c._r.t=8; }

// fn LDHLmn(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l, MMU.rb(c._r.pc)); c._r.pc++; c._r.m=3; c._r.t=12; }

// fn LDBCmA(c: &mut Cpu) { MMU.wb((c._r.b<<8)+c._r.c, c._r.a); c._r.m=2; c._r.t=8; }
// fn LDDEmA(c: &mut Cpu) { MMU.wb((c._r.d<<8)+c._r.e, c._r.a); c._r.m=2; c._r.t=8; }

// fn LDmmA(c: &mut Cpu) { MMU.wb(MMU.rw(c._r.pc), c._r.a); c._r.pc+=2; c._r.m=4; c._r.t=16; }

// fn LDABCm(c: &mut Cpu) { c._r.a=MMU.rb((c._r.b<<8)+c._r.c); c._r.m=2; c._r.t=8; }
// fn LDADEm(c: &mut Cpu) { c._r.a=MMU.rb((c._r.d<<8)+c._r.e); c._r.m=2; c._r.t=8; }

// fn LDAmm(c: &mut Cpu) { c._r.a=MMU.rb(MMU.rw(c._r.pc)); c._r.pc+=2; c._r.m=4; c._r.t=16; }

// fn LDBCnn(c: &mut Cpu) { c._r.c=MMU.rb(c._r.pc); c._r.b=MMU.rb(c._r.pc+1); c._r.pc+=2; c._r.m=3; c._r.t=12; }
// fn LDDEnn(c: &mut Cpu) { c._r.e=MMU.rb(c._r.pc); c._r.d=MMU.rb(c._r.pc+1); c._r.pc+=2; c._r.m=3; c._r.t=12; }
// fn LDHLnn(c: &mut Cpu) { c._r.l=MMU.rb(c._r.pc); c._r.h=MMU.rb(c._r.pc+1); c._r.pc+=2; c._r.m=3; c._r.t=12; }
// fn LDSPnn(c: &mut Cpu) { c._r.sp=MMU.rw(c._r.pc); c._r.pc+=2; c._r.m=3; c._r.t=12; }

// fn LDHLmm(c: &mut Cpu) { var i=MMU.rw(c._r.pc); c._r.pc+=2; c._r.l=MMU.rb(i); c._r.h=MMU.rb(i+1); c._r.m=5; c._r.t=20; }
// fn LDmmHL(c: &mut Cpu) { var i=MMU.rw(c._r.pc); c._r.pc+=2; MMU.ww(i,(c._r.h<<8)+c._r.l); c._r.m=5; c._r.t=20; }

// fn LDHLIA(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l, c._r.a); c._r.l=(c._r.l+1)&255; if(!c._r.l) c._r.h=(c._r.h+1)&255; c._r.m=2; c._r.t=8; }
// fn LDAHLI(c: &mut Cpu) { c._r.a=MMU.rb((c._r.h<<8)+c._r.l); c._r.l=(c._r.l+1)&255; if(!c._r.l) c._r.h=(c._r.h+1)&255; c._r.m=2; c._r.t=8; }

// fn LDHLDA(c: &mut Cpu) { MMU.wb((c._r.h<<8)+c._r.l, c._r.a); c._r.l=(c._r.l-1)&255; if(c._r.l==255) c._r.h=(c._r.h-1)&255; c._r.m=2; c._r.t=8; }
// fn LDAHLD(c: &mut Cpu) { c._r.a=MMU.rb((c._r.h<<8)+c._r.l); c._r.l=(c._r.l-1)&255; if(c._r.l==255) c._r.h=(c._r.h-1)&255; c._r.m=2; c._r.t=8; }

// fn LDAIOn(c: &mut Cpu) { c._r.a=MMU.rb(0xFF00+MMU.rb(c._r.pc)); c._r.pc++; c._r.m=3; c._r.t=12; }
// fn LDIOnA(c: &mut Cpu) { MMU.wb(0xFF00+MMU.rb(c._r.pc),c._r.a); c._r.pc++; c._r.m=3; c._r.t=12; }
// fn LDAIOC(c: &mut Cpu) { c._r.a=MMU.rb(0xFF00+c._r.c); c._r.m=2; c._r.t=8; }
// fn LDIOCA(c: &mut Cpu) { MMU.wb(0xFF00+c._r.c,c._r.a); c._r.m=2; c._r.t=8; }

// fn LDHLSPn(c: &mut Cpu) { var i=MMU.rb(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; i+=c._r.sp; c._r.h=(i>>8)&255; c._r.l=i&255; c._r.m=3; c._r.t=12; }

// fn SWAPr_b(c: &mut Cpu) { var tr=c._r.b; c._r.b=MMU.rb((c._r.h<<8)+c._r.l); MMU.wb((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
// fn SWAPr_c(c: &mut Cpu) { var tr=c._r.c; c._r.c=MMU.rb((c._r.h<<8)+c._r.l); MMU.wb((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
// fn SWAPr_d(c: &mut Cpu) { var tr=c._r.d; c._r.d=MMU.rb((c._r.h<<8)+c._r.l); MMU.wb((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
// fn SWAPr_e(c: &mut Cpu) { var tr=c._r.e; c._r.e=MMU.rb((c._r.h<<8)+c._r.l); MMU.wb((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
// fn SWAPr_h(c: &mut Cpu) { var tr=c._r.h; c._r.h=MMU.rb((c._r.h<<8)+c._r.l); MMU.wb((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
// fn SWAPr_l(c: &mut Cpu) { var tr=c._r.l; c._r.l=MMU.rb((c._r.h<<8)+c._r.l); MMU.wb((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
// fn SWAPr_a(c: &mut Cpu) { var tr=c._r.a; c._r.a=MMU.rb((c._r.h<<8)+c._r.l); MMU.wb((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }

// // // /*--- Data processing ---*/
// fn ADDr_b(c: &mut Cpu) { c._r.a+=c._r.b; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADDr_c(c: &mut Cpu) { c._r.a+=c._r.c; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADDr_d(c: &mut Cpu) { c._r.a+=c._r.d; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADDr_e(c: &mut Cpu) { c._r.a+=c._r.e; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADDr_h(c: &mut Cpu) { c._r.a+=c._r.h; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADDr_l(c: &mut Cpu) { c._r.a+=c._r.l; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADDr_a(c: &mut Cpu) { c._r.a+=c._r.a; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADDHL(c: &mut Cpu) { c._r.a+=MMU.rb((c._r.h<<8)+c._r.l); c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }
// fn ADDn(c: &mut Cpu) { c._r.a+=MMU.rb(c._r.pc); c._r.pc++; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }
// fn ADDHLBC(c: &mut Cpu) { var hl=(c._r.h<<8)+c._r.l; hl+=(c._r.b<<8)+c._r.c; if(hl>65535) c._r.f|=0x10; else c._r.f&=0xEF; c._r.h=(hl>>8)&255; c._r.l=hl&255; c._r.m=3; c._r.t=12; }
// fn ADDHLDE(c: &mut Cpu) { var hl=(c._r.h<<8)+c._r.l; hl+=(c._r.d<<8)+c._r.e; if(hl>65535) c._r.f|=0x10; else c._r.f&=0xEF; c._r.h=(hl>>8)&255; c._r.l=hl&255; c._r.m=3; c._r.t=12; }
// fn ADDHLHL(c: &mut Cpu) { var hl=(c._r.h<<8)+c._r.l; hl+=(c._r.h<<8)+c._r.l; if(hl>65535) c._r.f|=0x10; else c._r.f&=0xEF; c._r.h=(hl>>8)&255; c._r.l=hl&255; c._r.m=3; c._r.t=12; }
// fn ADDHLSP(c: &mut Cpu) { var hl=(c._r.h<<8)+c._r.l; hl+=c._r.sp; if(hl>65535) c._r.f|=0x10; else c._r.f&=0xEF; c._r.h=(hl>>8)&255; c._r.l=hl&255; c._r.m=3; c._r.t=12; }
// fn ADDSPn(c: &mut Cpu) { var i=MMU.rb(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; c._r.sp+=i; c._r.m=4; c._r.t=16; }

// fn ADCr_b(c: &mut Cpu) { c._r.a+=c._r.b; c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADCr_c(c: &mut Cpu) { c._r.a+=c._r.c; c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADCr_d(c: &mut Cpu) { c._r.a+=c._r.d; c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADCr_e(c: &mut Cpu) { c._r.a+=c._r.e; c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADCr_h(c: &mut Cpu) { c._r.a+=c._r.h; c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADCr_l(c: &mut Cpu) { c._r.a+=c._r.l; c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADCr_a(c: &mut Cpu) { c._r.a+=c._r.a; c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn ADCHL(c: &mut Cpu) { c._r.a+=MMU.rb((c._r.h<<8)+c._r.l); c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }
// fn ADCn(c: &mut Cpu) { c._r.a+=MMU.rb(c._r.pc); c._r.pc++; c._r.a+=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a); if(c._r.a>255) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }

// fn SUBr_b(c: &mut Cpu) { c._r.a-=c._r.b; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SUBr_c(c: &mut Cpu) { c._r.a-=c._r.c; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SUBr_d(c: &mut Cpu) { c._r.a-=c._r.d; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SUBr_e(c: &mut Cpu) { c._r.a-=c._r.e; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SUBr_h(c: &mut Cpu) { c._r.a-=c._r.h; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SUBr_l(c: &mut Cpu) { c._r.a-=c._r.l; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SUBr_a(c: &mut Cpu) { c._r.a-=c._r.a; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SUBHL(c: &mut Cpu) { c._r.a-=MMU.rb((c._r.h<<8)+c._r.l); c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }
// fn SUBn(c: &mut Cpu) { c._r.a-=MMU.rb(c._r.pc); c._r.pc++; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }

// fn SBCr_b(c: &mut Cpu) { c._r.a-=c._r.b; c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SBCr_c(c: &mut Cpu) { c._r.a-=c._r.c; c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SBCr_d(c: &mut Cpu) { c._r.a-=c._r.d; c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SBCr_e(c: &mut Cpu) { c._r.a-=c._r.e; c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SBCr_h(c: &mut Cpu) { c._r.a-=c._r.h; c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SBCr_l(c: &mut Cpu) { c._r.a-=c._r.l; c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SBCr_a(c: &mut Cpu) { c._r.a-=c._r.a; c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=1; c._r.t=4; }
// fn SBCHL(c: &mut Cpu) { c._r.a-=MMU.rb((c._r.h<<8)+c._r.l); c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }
// fn SBCn(c: &mut Cpu) { c._r.a-=MMU.rb(c._r.pc); c._r.pc++; c._r.a-=(c._r.f&0x10)?1:0; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }

// fn CPr_b(c: &mut Cpu) { var i=c._r.a; i-=c._r.b; c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=1; c._r.t=4; }
// fn CPr_c(c: &mut Cpu) { var i=c._r.a; i-=c._r.c; c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=1; c._r.t=4; }
// fn CPr_d(c: &mut Cpu) { var i=c._r.a; i-=c._r.d; c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=1; c._r.t=4; }
// fn CPr_e(c: &mut Cpu) { var i=c._r.a; i-=c._r.e; c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=1; c._r.t=4; }
// fn CPr_h(c: &mut Cpu) { var i=c._r.a; i-=c._r.h; c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=1; c._r.t=4; }
// fn CPr_l(c: &mut Cpu) { var i=c._r.a; i-=c._r.l; c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=1; c._r.t=4; }
// fn CPr_a(c: &mut Cpu) { var i=c._r.a; i-=c._r.a; c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=1; c._r.t=4; }
// fn CPHL(c: &mut Cpu) { var i=c._r.a; i-=MMU.rb((c._r.h<<8)+c._r.l); c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=2; c._r.t=8; }
// fn CPn(c: &mut Cpu) { var i=c._r.a; i-=MMU.rb(c._r.pc); c._r.pc++; c._ops.fz(i,1); if(i<0) c._r.f|=0x10; i&=255; c._r.m=2; c._r.t=8; }

// fn ANDr_b(c: &mut Cpu) { c._r.a&=c._r.b; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ANDr_c(c: &mut Cpu) { c._r.a&=c._r.c; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ANDr_d(c: &mut Cpu) { c._r.a&=c._r.d; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ANDr_e(c: &mut Cpu) { c._r.a&=c._r.e; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ANDr_h(c: &mut Cpu) { c._r.a&=c._r.h; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ANDr_l(c: &mut Cpu) { c._r.a&=c._r.l; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ANDr_a(c: &mut Cpu) { c._r.a&=c._r.a; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ANDHL(c: &mut Cpu) { c._r.a&=MMU.rb((c._r.h<<8)+c._r.l); c._r.a&=255; c._ops.fz(c._r.a); c._r.m=2; c._r.t=8; }
// fn ANDn(c: &mut Cpu) { c._r.a&=MMU.rb(c._r.pc); c._r.pc++; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=2; c._r.t=8; }

// fn ORr_b(c: &mut Cpu) { c._r.a|=c._r.b; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ORr_c(c: &mut Cpu) { c._r.a|=c._r.c; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ORr_d(c: &mut Cpu) { c._r.a|=c._r.d; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ORr_e(c: &mut Cpu) { c._r.a|=c._r.e; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ORr_h(c: &mut Cpu) { c._r.a|=c._r.h; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ORr_l(c: &mut Cpu) { c._r.a|=c._r.l; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ORr_a(c: &mut Cpu) { c._r.a|=c._r.a; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn ORHL(c: &mut Cpu) { c._r.a|=MMU.rb((c._r.h<<8)+c._r.l); c._r.a&=255; c._ops.fz(c._r.a); c._r.m=2; c._r.t=8; }
// fn ORn(c: &mut Cpu) { c._r.a|=MMU.rb(c._r.pc); c._r.pc++; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=2; c._r.t=8; }

// fn XORr_b(c: &mut Cpu) { c._r.a^=c._r.b; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn XORr_c(c: &mut Cpu) { c._r.a^=c._r.c; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn XORr_d(c: &mut Cpu) { c._r.a^=c._r.d; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn XORr_e(c: &mut Cpu) { c._r.a^=c._r.e; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn XORr_h(c: &mut Cpu) { c._r.a^=c._r.h; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn XORr_l(c: &mut Cpu) { c._r.a^=c._r.l; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn XORr_a(c: &mut Cpu) { c._r.a^=c._r.a; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn XORHL(c: &mut Cpu) { c._r.a^=MMU.rb((c._r.h<<8)+c._r.l); c._r.a&=255; c._ops.fz(c._r.a); c._r.m=2; c._r.t=8; }
// fn XORn(c: &mut Cpu) { c._r.a^=MMU.rb(c._r.pc); c._r.pc++; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=2; c._r.t=8; }

// fn INCr_b(c: &mut Cpu) { c._r.b++; c._r.b&=255; c._ops.fz(c._r.b); c._r.m=1; c._r.t=4; }
// fn INCr_c(c: &mut Cpu) { c._r.c++; c._r.c&=255; c._ops.fz(c._r.c); c._r.m=1; c._r.t=4; }
// fn INCr_d(c: &mut Cpu) { c._r.d++; c._r.d&=255; c._ops.fz(c._r.d); c._r.m=1; c._r.t=4; }
// fn INCr_e(c: &mut Cpu) { c._r.e++; c._r.e&=255; c._ops.fz(c._r.e); c._r.m=1; c._r.t=4; }
// fn INCr_h(c: &mut Cpu) { c._r.h++; c._r.h&=255; c._ops.fz(c._r.h); c._r.m=1; c._r.t=4; }
// fn INCr_l(c: &mut Cpu) { c._r.l++; c._r.l&=255; c._ops.fz(c._r.l); c._r.m=1; c._r.t=4; }
// fn INCr_a(c: &mut Cpu) { c._r.a++; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn INCHLm(c: &mut Cpu) { var i=MMU.rb((c._r.h<<8)+c._r.l)+1; i&=255; MMU.wb((c._r.h<<8)+c._r.l,i); c._ops.fz(i); c._r.m=3; c._r.t=12; }

// fn DECr_b(c: &mut Cpu) { c._r.b--; c._r.b&=255; c._ops.fz(c._r.b); c._r.m=1; c._r.t=4; }
// fn DECr_c(c: &mut Cpu) { c._r.c--; c._r.c&=255; c._ops.fz(c._r.c); c._r.m=1; c._r.t=4; }
// fn DECr_d(c: &mut Cpu) { c._r.d--; c._r.d&=255; c._ops.fz(c._r.d); c._r.m=1; c._r.t=4; }
// fn DECr_e(c: &mut Cpu) { c._r.e--; c._r.e&=255; c._ops.fz(c._r.e); c._r.m=1; c._r.t=4; }
// fn DECr_h(c: &mut Cpu) { c._r.h--; c._r.h&=255; c._ops.fz(c._r.h); c._r.m=1; c._r.t=4; }
// fn DECr_l(c: &mut Cpu) { c._r.l--; c._r.l&=255; c._ops.fz(c._r.l); c._r.m=1; c._r.t=4; }
// fn DECr_a(c: &mut Cpu) { c._r.a--; c._r.a&=255; c._ops.fz(c._r.a); c._r.m=1; c._r.t=4; }
// fn DECHLm(c: &mut Cpu) { var i=MMU.rb((c._r.h<<8)+c._r.l)-1; i&=255; MMU.wb((c._r.h<<8)+c._r.l,i); c._ops.fz(i); c._r.m=3; c._r.t=12; }

// fn INCBC(c: &mut Cpu) { c._r.c=(c._r.c+1)&255; if(!c._r.c) c._r.b=(c._r.b+1)&255; c._r.m=1; c._r.t=4; }
// fn INCDE(c: &mut Cpu) { c._r.e=(c._r.e+1)&255; if(!c._r.e) c._r.d=(c._r.d+1)&255; c._r.m=1; c._r.t=4; }
// fn INCHL(c: &mut Cpu) { c._r.l=(c._r.l+1)&255; if(!c._r.l) c._r.h=(c._r.h+1)&255; c._r.m=1; c._r.t=4; }
// fn INCSP(c: &mut Cpu) { c._r.sp=(c._r.sp+1)&65535; c._r.m=1; c._r.t=4; }

// fn DECBC(c: &mut Cpu) { c._r.c=(c._r.c-1)&255; if(c._r.c==255) c._r.b=(c._r.b-1)&255; c._r.m=1; c._r.t=4; }
// fn DECDE(c: &mut Cpu) { c._r.e=(c._r.e-1)&255; if(c._r.e==255) c._r.d=(c._r.d-1)&255; c._r.m=1; c._r.t=4; }
// fn DECHL(c: &mut Cpu) { c._r.l=(c._r.l-1)&255; if(c._r.l==255) c._r.h=(c._r.h-1)&255; c._r.m=1; c._r.t=4; }
// fn DECSP(c: &mut Cpu) { c._r.sp=(c._r.sp-1)&65535; c._r.m=1; c._r.t=4; }

// // // /*--- Bit manipulation ---*/
// fn BIT0b(c: &mut Cpu) { c._ops.fz(c._r.b&0x01); c._r.m=2; c._r.t=8; }
// fn BIT0c(c: &mut Cpu) { c._ops.fz(c._r.c&0x01); c._r.m=2; c._r.t=8; }
// fn BIT0d(c: &mut Cpu) { c._ops.fz(c._r.d&0x01); c._r.m=2; c._r.t=8; }
// fn BIT0e(c: &mut Cpu) { c._ops.fz(c._r.e&0x01); c._r.m=2; c._r.t=8; }
// fn BIT0h(c: &mut Cpu) { c._ops.fz(c._r.h&0x01); c._r.m=2; c._r.t=8; }
// fn BIT0l(c: &mut Cpu) { c._ops.fz(c._r.l&0x01); c._r.m=2; c._r.t=8; }
// fn BIT0a(c: &mut Cpu) { c._ops.fz(c._r.a&0x01); c._r.m=2; c._r.t=8; }
// fn BIT0m(c: &mut Cpu) { c._ops.fz(MMU.rb((c._r.h<<8)+c._r.l)&0x01); c._r.m=3; c._r.t=12; }

// fn BIT1b(c: &mut Cpu) { c._ops.fz(c._r.b&0x02); c._r.m=2; c._r.t=8; }
// fn BIT1c(c: &mut Cpu) { c._ops.fz(c._r.c&0x02); c._r.m=2; c._r.t=8; }
// fn BIT1d(c: &mut Cpu) { c._ops.fz(c._r.d&0x02); c._r.m=2; c._r.t=8; }
// fn BIT1e(c: &mut Cpu) { c._ops.fz(c._r.e&0x02); c._r.m=2; c._r.t=8; }
// fn BIT1h(c: &mut Cpu) { c._ops.fz(c._r.h&0x02); c._r.m=2; c._r.t=8; }
// fn BIT1l(c: &mut Cpu) { c._ops.fz(c._r.l&0x02); c._r.m=2; c._r.t=8; }
// fn BIT1a(c: &mut Cpu) { c._ops.fz(c._r.a&0x02); c._r.m=2; c._r.t=8; }
// fn BIT1m(c: &mut Cpu) { c._ops.fz(MMU.rb((c._r.h<<8)+c._r.l)&0x02); c._r.m=3; c._r.t=12; }

// fn BIT2b(c: &mut Cpu) { c._ops.fz(c._r.b&0x04); c._r.m=2; c._r.t=8; }
// fn BIT2c(c: &mut Cpu) { c._ops.fz(c._r.c&0x04); c._r.m=2; c._r.t=8; }
// fn BIT2d(c: &mut Cpu) { c._ops.fz(c._r.d&0x04); c._r.m=2; c._r.t=8; }
// fn BIT2e(c: &mut Cpu) { c._ops.fz(c._r.e&0x04); c._r.m=2; c._r.t=8; }
// fn BIT2h(c: &mut Cpu) { c._ops.fz(c._r.h&0x04); c._r.m=2; c._r.t=8; }
// fn BIT2l(c: &mut Cpu) { c._ops.fz(c._r.l&0x04); c._r.m=2; c._r.t=8; }
// fn BIT2a(c: &mut Cpu) { c._ops.fz(c._r.a&0x04); c._r.m=2; c._r.t=8; }
// fn BIT2m(c: &mut Cpu) { c._ops.fz(MMU.rb((c._r.h<<8)+c._r.l)&0x04); c._r.m=3; c._r.t=12; }

// fn BIT3b(c: &mut Cpu) { c._ops.fz(c._r.b&0x08); c._r.m=2; c._r.t=8; }
// fn BIT3c(c: &mut Cpu) { c._ops.fz(c._r.c&0x08); c._r.m=2; c._r.t=8; }
// fn BIT3d(c: &mut Cpu) { c._ops.fz(c._r.d&0x08); c._r.m=2; c._r.t=8; }
// fn BIT3e(c: &mut Cpu) { c._ops.fz(c._r.e&0x08); c._r.m=2; c._r.t=8; }
// fn BIT3h(c: &mut Cpu) { c._ops.fz(c._r.h&0x08); c._r.m=2; c._r.t=8; }
// fn BIT3l(c: &mut Cpu) { c._ops.fz(c._r.l&0x08); c._r.m=2; c._r.t=8; }
// fn BIT3a(c: &mut Cpu) { c._ops.fz(c._r.a&0x08); c._r.m=2; c._r.t=8; }
// fn BIT3m(c: &mut Cpu) { c._ops.fz(MMU.rb((c._r.h<<8)+c._r.l)&0x08); c._r.m=3; c._r.t=12; }

// fn BIT4b(c: &mut Cpu) { c._ops.fz(c._r.b&0x10); c._r.m=2; c._r.t=8; }
// fn BIT4c(c: &mut Cpu) { c._ops.fz(c._r.c&0x10); c._r.m=2; c._r.t=8; }
// fn BIT4d(c: &mut Cpu) { c._ops.fz(c._r.d&0x10); c._r.m=2; c._r.t=8; }
// fn BIT4e(c: &mut Cpu) { c._ops.fz(c._r.e&0x10); c._r.m=2; c._r.t=8; }
// fn BIT4h(c: &mut Cpu) { c._ops.fz(c._r.h&0x10); c._r.m=2; c._r.t=8; }
// fn BIT4l(c: &mut Cpu) { c._ops.fz(c._r.l&0x10); c._r.m=2; c._r.t=8; }
// fn BIT4a(c: &mut Cpu) { c._ops.fz(c._r.a&0x10); c._r.m=2; c._r.t=8; }
// fn BIT4m(c: &mut Cpu) { c._ops.fz(MMU.rb((c._r.h<<8)+c._r.l)&0x10); c._r.m=3; c._r.t=12; }

// fn BIT5b(c: &mut Cpu) { c._ops.fz(c._r.b&0x20); c._r.m=2; c._r.t=8; }
// fn BIT5c(c: &mut Cpu) { c._ops.fz(c._r.c&0x20); c._r.m=2; c._r.t=8; }
// fn BIT5d(c: &mut Cpu) { c._ops.fz(c._r.d&0x20); c._r.m=2; c._r.t=8; }
// fn BIT5e(c: &mut Cpu) { c._ops.fz(c._r.e&0x20); c._r.m=2; c._r.t=8; }
// fn BIT5h(c: &mut Cpu) { c._ops.fz(c._r.h&0x20); c._r.m=2; c._r.t=8; }
// fn BIT5l(c: &mut Cpu) { c._ops.fz(c._r.l&0x20); c._r.m=2; c._r.t=8; }
// fn BIT5a(c: &mut Cpu) { c._ops.fz(c._r.a&0x20); c._r.m=2; c._r.t=8; }
// fn BIT5m(c: &mut Cpu) { c._ops.fz(MMU.rb((c._r.h<<8)+c._r.l)&0x20); c._r.m=3; c._r.t=12; }

// fn BIT6b(c: &mut Cpu) { c._ops.fz(c._r.b&0x40); c._r.m=2; c._r.t=8; }
// fn BIT6c(c: &mut Cpu) { c._ops.fz(c._r.c&0x40); c._r.m=2; c._r.t=8; }
// fn BIT6d(c: &mut Cpu) { c._ops.fz(c._r.d&0x40); c._r.m=2; c._r.t=8; }
// fn BIT6e(c: &mut Cpu) { c._ops.fz(c._r.e&0x40); c._r.m=2; c._r.t=8; }
// fn BIT6h(c: &mut Cpu) { c._ops.fz(c._r.h&0x40); c._r.m=2; c._r.t=8; }
// fn BIT6l(c: &mut Cpu) { c._ops.fz(c._r.l&0x40); c._r.m=2; c._r.t=8; }
// fn BIT6a(c: &mut Cpu) { c._ops.fz(c._r.a&0x40); c._r.m=2; c._r.t=8; }
// fn BIT6m(c: &mut Cpu) { c._ops.fz(MMU.rb((c._r.h<<8)+c._r.l)&0x40); c._r.m=3; c._r.t=12; }

// fn BIT7b(c: &mut Cpu) { c._ops.fz(c._r.b&0x80); c._r.m=2; c._r.t=8; }
// fn BIT7c(c: &mut Cpu) { c._ops.fz(c._r.c&0x80); c._r.m=2; c._r.t=8; }
// fn BIT7d(c: &mut Cpu) { c._ops.fz(c._r.d&0x80); c._r.m=2; c._r.t=8; }
// fn BIT7e(c: &mut Cpu) { c._ops.fz(c._r.e&0x80); c._r.m=2; c._r.t=8; }
// fn BIT7h(c: &mut Cpu) { c._ops.fz(c._r.h&0x80); c._r.m=2; c._r.t=8; }
// fn BIT7l(c: &mut Cpu) { c._ops.fz(c._r.l&0x80); c._r.m=2; c._r.t=8; }
// fn BIT7a(c: &mut Cpu) { c._ops.fz(c._r.a&0x80); c._r.m=2; c._r.t=8; }
// fn BIT7m(c: &mut Cpu) { c._ops.fz(MMU.rb((c._r.h<<8)+c._r.l)&0x80); c._r.m=3; c._r.t=12; }

// TODO:

// fn RLA(c: &mut Cpu) { var ci=c._r.f&0x10?1:0; var co=c._r.a&0x80?0x10:0; c._r.a=(c._r.a<<1)+ci; c._r.a&=255; c._r.f=(c._r.f&0xEF)+co; c._r.m=1; c._r.t=4; }
// fn RLCA(c: &mut Cpu) { var ci=c._r.a&0x80?1:0; var co=c._r.a&0x80?0x10:0; c._r.a=(c._r.a<<1)+ci; c._r.a&=255; c._r.f=(c._r.f&0xEF)+co; c._r.m=1; c._r.t=4; }
// fn RRA(c: &mut Cpu) { var ci=c._r.f&0x10?0x80:0; var co=c._r.a&1?0x10:0; c._r.a=(c._r.a>>1)+ci; c._r.a&=255; c._r.f=(c._r.f&0xEF)+co; c._r.m=1; c._r.t=4; }
// fn RRCA(c: &mut Cpu) { var ci=c._r.a&1?0x80:0; var co=c._r.a&1?0x10:0; c._r.a=(c._r.a>>1)+ci; c._r.a&=255; c._r.f=(c._r.f&0xEF)+co; c._r.m=1; c._r.t=4; }

// fn RLr_b(c: &mut Cpu) { var ci=c._r.f&0x10?1:0; var co=c._r.b&0x80?0x10:0; c._r.b=(c._r.b<<1)+ci; c._r.b&=255; c._ops.fz(c._r.b); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLr_c(c: &mut Cpu) { var ci=c._r.f&0x10?1:0; var co=c._r.c&0x80?0x10:0; c._r.c=(c._r.c<<1)+ci; c._r.c&=255; c._ops.fz(c._r.c); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLr_d(c: &mut Cpu) { var ci=c._r.f&0x10?1:0; var co=c._r.d&0x80?0x10:0; c._r.d=(c._r.d<<1)+ci; c._r.d&=255; c._ops.fz(c._r.d); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLr_e(c: &mut Cpu) { var ci=c._r.f&0x10?1:0; var co=c._r.e&0x80?0x10:0; c._r.e=(c._r.e<<1)+ci; c._r.e&=255; c._ops.fz(c._r.e); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLr_h(c: &mut Cpu) { var ci=c._r.f&0x10?1:0; var co=c._r.h&0x80?0x10:0; c._r.h=(c._r.h<<1)+ci; c._r.h&=255; c._ops.fz(c._r.h); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLr_l(c: &mut Cpu) { var ci=c._r.f&0x10?1:0; var co=c._r.l&0x80?0x10:0; c._r.l=(c._r.l<<1)+ci; c._r.l&=255; c._ops.fz(c._r.l); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLr_a(c: &mut Cpu) { var ci=c._r.f&0x10?1:0; var co=c._r.a&0x80?0x10:0; c._r.a=(c._r.a<<1)+ci; c._r.a&=255; c._ops.fz(c._r.a); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLHL(c: &mut Cpu) { var i=MMU.rb((c._r.h<<8)+c._r.l); var ci=c._r.f&0x10?1:0; var co=i&0x80?0x10:0; i=(i<<1)+ci; i&=255; c._ops.fz(i); MMU.wb((c._r.h<<8)+c._r.l,i); c._r.f=(c._r.f&0xEF)+co; c._r.m=4; c._r.t=16; }

// fn RLCr_b(c: &mut Cpu) { var ci=c._r.b&0x80?1:0; var co=c._r.b&0x80?0x10:0; c._r.b=(c._r.b<<1)+ci; c._r.b&=255; c._ops.fz(c._r.b); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLCr_c(c: &mut Cpu) { var ci=c._r.c&0x80?1:0; var co=c._r.c&0x80?0x10:0; c._r.c=(c._r.c<<1)+ci; c._r.c&=255; c._ops.fz(c._r.c); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLCr_d(c: &mut Cpu) { var ci=c._r.d&0x80?1:0; var co=c._r.d&0x80?0x10:0; c._r.d=(c._r.d<<1)+ci; c._r.d&=255; c._ops.fz(c._r.d); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLCr_e(c: &mut Cpu) { var ci=c._r.e&0x80?1:0; var co=c._r.e&0x80?0x10:0; c._r.e=(c._r.e<<1)+ci; c._r.e&=255; c._ops.fz(c._r.e); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLCr_h(c: &mut Cpu) { var ci=c._r.h&0x80?1:0; var co=c._r.h&0x80?0x10:0; c._r.h=(c._r.h<<1)+ci; c._r.h&=255; c._ops.fz(c._r.h); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLCr_l(c: &mut Cpu) { var ci=c._r.l&0x80?1:0; var co=c._r.l&0x80?0x10:0; c._r.l=(c._r.l<<1)+ci; c._r.l&=255; c._ops.fz(c._r.l); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLCr_a(c: &mut Cpu) { var ci=c._r.a&0x80?1:0; var co=c._r.a&0x80?0x10:0; c._r.a=(c._r.a<<1)+ci; c._r.a&=255; c._ops.fz(c._r.a); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RLCHL(c: &mut Cpu) { var i=MMU.rb((c._r.h<<8)+c._r.l); var ci=i&0x80?1:0; var co=i&0x80?0x10:0; i=(i<<1)+ci; i&=255; c._ops.fz(i); MMU.wb((c._r.h<<8)+c._r.l,i); c._r.f=(c._r.f&0xEF)+co; c._r.m=4; c._r.t=16; }

// fn RRr_b(c: &mut Cpu) { var ci=c._r.f&0x10?0x80:0; var co=c._r.b&1?0x10:0; c._r.b=(c._r.b>>1)+ci; c._r.b&=255; c._ops.fz(c._r.b); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRr_c(c: &mut Cpu) { var ci=c._r.f&0x10?0x80:0; var co=c._r.c&1?0x10:0; c._r.c=(c._r.c>>1)+ci; c._r.c&=255; c._ops.fz(c._r.c); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRr_d(c: &mut Cpu) { var ci=c._r.f&0x10?0x80:0; var co=c._r.d&1?0x10:0; c._r.d=(c._r.d>>1)+ci; c._r.d&=255; c._ops.fz(c._r.d); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRr_e(c: &mut Cpu) { var ci=c._r.f&0x10?0x80:0; var co=c._r.e&1?0x10:0; c._r.e=(c._r.e>>1)+ci; c._r.e&=255; c._ops.fz(c._r.e); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRr_h(c: &mut Cpu) { var ci=c._r.f&0x10?0x80:0; var co=c._r.h&1?0x10:0; c._r.h=(c._r.h>>1)+ci; c._r.h&=255; c._ops.fz(c._r.h); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRr_l(c: &mut Cpu) { var ci=c._r.f&0x10?0x80:0; var co=c._r.l&1?0x10:0; c._r.l=(c._r.l>>1)+ci; c._r.l&=255; c._ops.fz(c._r.l); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRr_a(c: &mut Cpu) { var ci=c._r.f&0x10?0x80:0; var co=c._r.a&1?0x10:0; c._r.a=(c._r.a>>1)+ci; c._r.a&=255; c._ops.fz(c._r.a); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRHL(c: &mut Cpu) { var i=MMU.rb((c._r.h<<8)+c._r.l); var ci=c._r.f&0x10?0x80:0; var co=i&1?0x10:0; i=(i>>1)+ci; i&=255; MMU.wb((c._r.h<<8)+c._r.l,i); c._ops.fz(i); c._r.f=(c._r.f&0xEF)+co; c._r.m=4; c._r.t=16; }

// fn RRCr_b(c: &mut Cpu) { var ci=c._r.b&1?0x80:0; var co=c._r.b&1?0x10:0; c._r.b=(c._r.b>>1)+ci; c._r.b&=255; c._ops.fz(c._r.b); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRCr_c(c: &mut Cpu) { var ci=c._r.c&1?0x80:0; var co=c._r.c&1?0x10:0; c._r.c=(c._r.c>>1)+ci; c._r.c&=255; c._ops.fz(c._r.c); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRCr_d(c: &mut Cpu) { var ci=c._r.d&1?0x80:0; var co=c._r.d&1?0x10:0; c._r.d=(c._r.d>>1)+ci; c._r.d&=255; c._ops.fz(c._r.d); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRCr_e(c: &mut Cpu) { var ci=c._r.e&1?0x80:0; var co=c._r.e&1?0x10:0; c._r.e=(c._r.e>>1)+ci; c._r.e&=255; c._ops.fz(c._r.e); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRCr_h(c: &mut Cpu) { var ci=c._r.h&1?0x80:0; var co=c._r.h&1?0x10:0; c._r.h=(c._r.h>>1)+ci; c._r.h&=255; c._ops.fz(c._r.h); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRCr_l(c: &mut Cpu) { var ci=c._r.l&1?0x80:0; var co=c._r.l&1?0x10:0; c._r.l=(c._r.l>>1)+ci; c._r.l&=255; c._ops.fz(c._r.l); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRCr_a(c: &mut Cpu) { var ci=c._r.a&1?0x80:0; var co=c._r.a&1?0x10:0; c._r.a=(c._r.a>>1)+ci; c._r.a&=255; c._ops.fz(c._r.a); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn RRCHL(c: &mut Cpu) { var i=MMU.rb((c._r.h<<8)+c._r.l); var ci=i&1?0x80:0; var co=i&1?0x10:0; i=(i>>1)+ci; i&=255; MMU.wb((c._r.h<<8)+c._r.l,i); c._ops.fz(i); c._r.f=(c._r.f&0xEF)+co; c._r.m=4; c._r.t=16; }

// fn SLAr_b(c: &mut Cpu) { var co=c._r.b&0x80?0x10:0; c._r.b=(c._r.b<<1)&255; c._ops.fz(c._r.b); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLAr_c(c: &mut Cpu) { var co=c._r.c&0x80?0x10:0; c._r.c=(c._r.c<<1)&255; c._ops.fz(c._r.c); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLAr_d(c: &mut Cpu) { var co=c._r.d&0x80?0x10:0; c._r.d=(c._r.d<<1)&255; c._ops.fz(c._r.d); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLAr_e(c: &mut Cpu) { var co=c._r.e&0x80?0x10:0; c._r.e=(c._r.e<<1)&255; c._ops.fz(c._r.e); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLAr_h(c: &mut Cpu) { var co=c._r.h&0x80?0x10:0; c._r.h=(c._r.h<<1)&255; c._ops.fz(c._r.h); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLAr_l(c: &mut Cpu) { var co=c._r.l&0x80?0x10:0; c._r.l=(c._r.l<<1)&255; c._ops.fz(c._r.l); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLAr_a(c: &mut Cpu) { var co=c._r.a&0x80?0x10:0; c._r.a=(c._r.a<<1)&255; c._ops.fz(c._r.a); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }

// fn SLLr_b(c: &mut Cpu) { var co=c._r.b&0x80?0x10:0; c._r.b=(c._r.b<<1)&255+1; c._ops.fz(c._r.b); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLLr_c(c: &mut Cpu) { var co=c._r.c&0x80?0x10:0; c._r.c=(c._r.c<<1)&255+1; c._ops.fz(c._r.c); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLLr_d(c: &mut Cpu) { var co=c._r.d&0x80?0x10:0; c._r.d=(c._r.d<<1)&255+1; c._ops.fz(c._r.d); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLLr_e(c: &mut Cpu) { var co=c._r.e&0x80?0x10:0; c._r.e=(c._r.e<<1)&255+1; c._ops.fz(c._r.e); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLLr_h(c: &mut Cpu) { var co=c._r.h&0x80?0x10:0; c._r.h=(c._r.h<<1)&255+1; c._ops.fz(c._r.h); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLLr_l(c: &mut Cpu) { var co=c._r.l&0x80?0x10:0; c._r.l=(c._r.l<<1)&255+1; c._ops.fz(c._r.l); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SLLr_a(c: &mut Cpu) { var co=c._r.a&0x80?0x10:0; c._r.a=(c._r.a<<1)&255+1; c._ops.fz(c._r.a); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }

// fn SRAr_b(c: &mut Cpu) { var ci=c._r.b&0x80; var co=c._r.b&1?0x10:0; c._r.b=((c._r.b>>1)+ci)&255; c._ops.fz(c._r.b); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRAr_c(c: &mut Cpu) { var ci=c._r.c&0x80; var co=c._r.c&1?0x10:0; c._r.c=((c._r.c>>1)+ci)&255; c._ops.fz(c._r.c); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRAr_d(c: &mut Cpu) { var ci=c._r.d&0x80; var co=c._r.d&1?0x10:0; c._r.d=((c._r.d>>1)+ci)&255; c._ops.fz(c._r.d); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRAr_e(c: &mut Cpu) { var ci=c._r.e&0x80; var co=c._r.e&1?0x10:0; c._r.e=((c._r.e>>1)+ci)&255; c._ops.fz(c._r.e); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRAr_h(c: &mut Cpu) { var ci=c._r.h&0x80; var co=c._r.h&1?0x10:0; c._r.h=((c._r.h>>1)+ci)&255; c._ops.fz(c._r.h); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRAr_l(c: &mut Cpu) { var ci=c._r.l&0x80; var co=c._r.l&1?0x10:0; c._r.l=((c._r.l>>1)+ci)&255; c._ops.fz(c._r.l); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRAr_a(c: &mut Cpu) { var ci=c._r.a&0x80; var co=c._r.a&1?0x10:0; c._r.a=((c._r.a>>1)+ci)&255; c._ops.fz(c._r.a); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }

// fn SRLr_b(c: &mut Cpu) { var co=c._r.b&1?0x10:0; c._r.b=(c._r.b>>1)&255; c._ops.fz(c._r.b); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRLr_c(c: &mut Cpu) { var co=c._r.c&1?0x10:0; c._r.c=(c._r.c>>1)&255; c._ops.fz(c._r.c); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRLr_d(c: &mut Cpu) { var co=c._r.d&1?0x10:0; c._r.d=(c._r.d>>1)&255; c._ops.fz(c._r.d); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRLr_e(c: &mut Cpu) { var co=c._r.e&1?0x10:0; c._r.e=(c._r.e>>1)&255; c._ops.fz(c._r.e); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRLr_h(c: &mut Cpu) { var co=c._r.h&1?0x10:0; c._r.h=(c._r.h>>1)&255; c._ops.fz(c._r.h); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRLr_l(c: &mut Cpu) { var co=c._r.l&1?0x10:0; c._r.l=(c._r.l>>1)&255; c._ops.fz(c._r.l); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }
// fn SRLr_a(c: &mut Cpu) { var co=c._r.a&1?0x10:0; c._r.a=(c._r.a>>1)&255; c._ops.fz(c._r.a); c._r.f=(c._r.f&0xEF)+co; c._r.m=2; c._r.t=8; }

// fn CPL(c: &mut Cpu) { c._r.a = (~c._r.a)&255; c._ops.fz(c._r.a,1); c._r.m=1; c._r.t=4; }
// fn NEG(c: &mut Cpu) { c._r.a=0-c._r.a; c._ops.fz(c._r.a,1); if(c._r.a<0) c._r.f|=0x10; c._r.a&=255; c._r.m=2; c._r.t=8; }

// fn CCF(c: &mut Cpu) { var ci=c._r.f&0x10?0:0x10; c._r.f=(c._r.f&0xEF)+ci; c._r.m=1; c._r.t=4; }
// fn SCF(c: &mut Cpu) { c._r.f|=0x10; c._r.m=1; c._r.t=4; }

// // // /*--- Stack ---*/
// fn PUSHBC(c: &mut Cpu) { c._r.sp--; MMU.wb(c._r.sp,c._r.b); c._r.sp--; MMU.wb(c._r.sp,c._r.c); c._r.m=3; c._r.t=12; }
// fn PUSHDE(c: &mut Cpu) { c._r.sp--; MMU.wb(c._r.sp,c._r.d); c._r.sp--; MMU.wb(c._r.sp,c._r.e); c._r.m=3; c._r.t=12; }
// fn PUSHHL(c: &mut Cpu) { c._r.sp--; MMU.wb(c._r.sp,c._r.h); c._r.sp--; MMU.wb(c._r.sp,c._r.l); c._r.m=3; c._r.t=12; }
// fn PUSHAF(c: &mut Cpu) { c._r.sp--; MMU.wb(c._r.sp,c._r.a); c._r.sp--; MMU.wb(c._r.sp,c._r.f); c._r.m=3; c._r.t=12; }

// fn POPBC(c: &mut Cpu) { c._r.c=MMU.rb(c._r.sp); c._r.sp++; c._r.b=MMU.rb(c._r.sp); c._r.sp++; c._r.m=3; c._r.t=12; }
// fn POPDE(c: &mut Cpu) { c._r.e=MMU.rb(c._r.sp); c._r.sp++; c._r.d=MMU.rb(c._r.sp); c._r.sp++; c._r.m=3; c._r.t=12; }
// fn POPHL(c: &mut Cpu) { c._r.l=MMU.rb(c._r.sp); c._r.sp++; c._r.h=MMU.rb(c._r.sp); c._r.sp++; c._r.m=3; c._r.t=12; }
// fn POPAF(c: &mut Cpu) { c._r.f=MMU.rb(c._r.sp); c._r.sp++; c._r.a=MMU.rb(c._r.sp); c._r.sp++; c._r.m=3; c._r.t=12; }

// // // /*--- Jump ---*/
// fn JPnn(c: &mut Cpu) { c._r.pc = MMU.rw(c._r.pc); c._r.m=3; c._r.t=12; }
// fn JPHL(c: &mut Cpu) { c._r.pc=c._r.hl; c._r.m=1; c._r.t=4; }
// fn JPNZnn(c: &mut Cpu) { c._r.m=3; c._r.t=12; if((c._r.f&0x80)==0x00) { c._r.pc=MMU.rw(c._r.pc); c._r.m++; c._r.t+=4; } else c._r.pc+=2; }
// fn JPZnn(c: &mut Cpu)  { c._r.m=3; c._r.t=12; if((c._r.f&0x80)==0x80) { c._r.pc=MMU.rw(c._r.pc); c._r.m++; c._r.t+=4; } else c._r.pc+=2; }
// fn JPNCnn(c: &mut Cpu) { c._r.m=3; c._r.t=12; if((c._r.f&0x10)==0x00) { c._r.pc=MMU.rw(c._r.pc); c._r.m++; c._r.t+=4; } else c._r.pc+=2; }
// fn JPCnn(c: &mut Cpu)  { c._r.m=3; c._r.t=12; if((c._r.f&0x10)==0x10) { c._r.pc=MMU.rw(c._r.pc); c._r.m++; c._r.t+=4; } else c._r.pc+=2; }

// fn JRn(c: &mut Cpu) { var i=MMU.rb(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; c._r.m=2; c._r.t=8; c._r.pc+=i; c._r.m++; c._r.t+=4; }
// fn JRNZn(c: &mut Cpu) { var i=MMU.rb(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; c._r.m=2; c._r.t=8; if((c._r.f&0x80)==0x00) { c._r.pc+=i; c._r.m++; c._r.t+=4; } }
// fn JRZn(c: &mut Cpu)  { var i=MMU.rb(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; c._r.m=2; c._r.t=8; if((c._r.f&0x80)==0x80) { c._r.pc+=i; c._r.m++; c._r.t+=4; } }
// fn JRNCn(c: &mut Cpu) { var i=MMU.rb(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; c._r.m=2; c._r.t=8; if((c._r.f&0x10)==0x00) { c._r.pc+=i; c._r.m++; c._r.t+=4; } }
// fn JRCn(c: &mut Cpu)  { var i=MMU.rb(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; c._r.m=2; c._r.t=8; if((c._r.f&0x10)==0x10) { c._r.pc+=i; c._r.m++; c._r.t+=4; } }

// fn DJNZn(c: &mut Cpu) { var i=MMU.rb(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; c._r.m=2; c._r.t=8; c._r.b--; if(c._r.b) { c._r.pc+=i; c._r.m++; c._r.t+=4; } }

// fn CALLnn(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc+2); c._r.pc=MMU.rw(c._r.pc); c._r.m=5; c._r.t=20; }
// fn CALLNZnn(c: &mut Cpu) { c._r.m=3; c._r.t=12; if((c._r.f&0x80)==0x00) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc+2); c._r.pc=MMU.rw(c._r.pc); c._r.m+=2; c._r.t+=8; } else c._r.pc+=2; }
// fn CALLZnn(c: &mut Cpu) { c._r.m=3; c._r.t=12; if((c._r.f&0x80)==0x80) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc+2); c._r.pc=MMU.rw(c._r.pc); c._r.m+=2; c._r.t+=8; } else c._r.pc+=2; }
// fn CALLNCnn(c: &mut Cpu) { c._r.m=3; c._r.t=12; if((c._r.f&0x10)==0x00) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc+2); c._r.pc=MMU.rw(c._r.pc); c._r.m+=2; c._r.t+=8; } else c._r.pc+=2; }
// fn CALLCnn(c: &mut Cpu) { c._r.m=3; c._r.t=12; if((c._r.f&0x10)==0x10) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc+2); c._r.pc=MMU.rw(c._r.pc); c._r.m+=2; c._r.t+=8; } else c._r.pc+=2; }

// fn RET(c: &mut Cpu) { c._r.pc=MMU.rw(c._r.sp); c._r.sp+=2; c._r.m=3; c._r.t=12; }
// fn RETI(c: &mut Cpu) { c._r.ime=1; c._r.pc=MMU.rw(c._r.sp); c._r.sp+=2; c._r.m=3; c._r.t=12; }
// fn RETNZ(c: &mut Cpu) { c._r.m=1; c._r.t=4; if((c._r.f&0x80)==0x00) { c._r.pc=MMU.rw(c._r.sp); c._r.sp+=2; c._r.m+=2; c._r.t+=8; } }
// fn RETZ(c: &mut Cpu) { c._r.m=1; c._r.t=4; if((c._r.f&0x80)==0x80) { c._r.pc=MMU.rw(c._r.sp); c._r.sp+=2; c._r.m+=2; c._r.t+=8; } }
// fn RETNC(c: &mut Cpu) { c._r.m=1; c._r.t=4; if((c._r.f&0x10)==0x00) { c._r.pc=MMU.rw(c._r.sp); c._r.sp+=2; c._r.m+=2; c._r.t+=8; } }
// fn RETC(c: &mut Cpu) { c._r.m=1; c._r.t=4; if((c._r.f&0x10)==0x10) { c._r.pc=MMU.rw(c._r.sp); c._r.sp+=2; c._r.m+=2; c._r.t+=8; } }

// fn RST00(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x00; c._r.m=3; c._r.t=12; }
// fn RST08(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x08; c._r.m=3; c._r.t=12; }
// fn RST10(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x10; c._r.m=3; c._r.t=12; }
// fn RST18(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x18; c._r.m=3; c._r.t=12; }
// fn RST20(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x20; c._r.m=3; c._r.t=12; }
// fn RST28(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x28; c._r.m=3; c._r.t=12; }
// fn RST30(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x30; c._r.m=3; c._r.t=12; }
// fn RST38(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x38; c._r.m=3; c._r.t=12; }
// fn RST40(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x40; c._r.m=3; c._r.t=12; }
// fn RST48(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x48; c._r.m=3; c._r.t=12; }
// fn RST50(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x50; c._r.m=3; c._r.t=12; }
// fn RST58(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x58; c._r.m=3; c._r.t=12; }
// fn RST60(c: &mut Cpu) { c._r.sp-=2; MMU.ww(c._r.sp,c._r.pc); c._r.pc=0x60; c._r.m=3; c._r.t=12; }

// fn HALT(c: &mut Cpu) { c._halt=1; c._r.m=1; c._r.t=4; }

// fn DI(c: &mut Cpu) { c._r.ime=0; c._r.m=1; c._r.t=4; }
// fn EI(c: &mut Cpu) { c._r.ime=1; c._r.m=1; c._r.t=4; }

// // // /*--- Helper functions ---*/
// fn fz(c: &mut Cpui,as) { c._r.f=0; if(!(i&255)) c._r.f|=128; c._r.f|=as?0x40:0; }
// fn MAPcb(c: &mut Cpu) {
// // //   var i=MMU.rb(c._r.pc); c._r.pc++;
// // //   c._r.pc &= 65535;
// // //   if(c._cbmap[i]) c._cbmap[i]();
// // //   else alert(i);
// // // }

// fn XX(c: &mut Cpu) {
// // //   /*Undefined map entry*/
// // //   var opc = c._r.pc-1;
// // //   alert('Unimplemented instruction at $'+opc.toString(16)+', stopping.');
// // //   c._stop=1;
// // // }
