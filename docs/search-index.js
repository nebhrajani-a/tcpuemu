var searchIndex = JSON.parse('{\
"tcpuemu":{"doc":"This crate implements an emulator for the TinyCPU, a small …","t":[13,13,13,13,3,13,3,13,13,13,13,13,13,13,4,3,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,12,11,11,12,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["ADD","AND","CPAM","CPPA","Cpu","INV","Instruction","JE","JG","JL","JU","LDI","LDM","OR","Opcode","Registers","STM","SWAB","SWMB","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","default","dump_state","execute","from","from","from","from","imm","into","into","into","into","mem","new","new","opcode","r_a","r_b","r_m","r_p","regs","run","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id"],"q":["tcpuemu","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","Implements the CPU, and keeps track of its state.","","Implements the decoding and execution of each instruction …","","","","","","","","Opcode for each instruction type.","Contains the four registers of TinyCPU. These are intended …","","","","","","","","","","","","","Print out the state of the CPU registers.","Execute an instruction and update the CPU state.","","","","","","","","","","","Constructor for the CPU: the current implementation …","Decode an instruction and for LDI, an immediate with the …","","","","","","","Run the CPU by repeatedly calling <code>Instruction::execute</code> and …","","","","","","","","","","","",""],"i":[1,1,1,1,0,1,0,1,1,1,1,1,1,1,0,0,1,1,1,2,3,1,4,2,3,1,4,4,2,3,2,3,1,4,3,2,3,1,4,2,2,3,3,4,4,4,4,2,2,2,3,1,4,2,3,1,4,2,3,1,4],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["registers",3]],[[]],[[["cpu",3]]],[[]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[]],null,[[["option",4,[["vec",3,[["wrapping",3,[["u8",15]]]]]]]]],[[["wrapping",3,[["u8",15]]]]],null,null,null,null,null,null,[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[4,"Opcode"],[3,"Cpu"],[3,"Instruction"],[3,"Registers"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};