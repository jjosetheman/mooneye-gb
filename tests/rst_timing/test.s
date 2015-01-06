; RST is expected to have the following timing:
; t = 0: instruction decoding
; t = 1: internal delay
; t = 2: PC push: memory access for high byte
; t = 3: PC push: memory access for low byte

.incdir "../common"
.include "common.i"

  di

  ; set first $20 bytes of VRAM to $81, so we
  ; have a known value when reading results
  wait_vblank
  ld hl, VRAM
  ld bc, $20
  ld a, $81
  call memset

  run_hiram_test

test_finish:
  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  save_results
  assert_b $81
  assert_c $9E
  assert_d $FF
  assert_e $BD
  jp print_results

hiram_test:
  ld sp, OAM+$10

  start_oam_dma $80
  ld a, 38
- dec a
  jr nz, -
  ld hl, $FF80 + (finish_round1 - hiram_test)
  nops 2

  rst $38
  ; OAM is accessible at t=3, so we expect to see
  ; incorrect (= $81 written by OAM DMA) high byte, but correct low byte

finish_round1:
  nops 2
  pop bc

  start_oam_dma $80
  ld a, 38
- dec a
  jr nz, -
  ld hl, $FF80 + (finish_round2 - hiram_test)
  nops 3

  rst $38
  ; OAM is accessible at t=2, so we expect to see
  ; correct high byte and low byte

finish_round2:
  nops 2
  pop de

  jp test_finish

.org $38
  jp hl