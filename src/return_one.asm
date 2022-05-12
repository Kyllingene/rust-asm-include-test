default rel
section .text
global return_one
return_one:
	push rbp
	mov rbp, rsp
	mov rax, 0x1
	mov rsp, rbp
	pop rbp
	ret