fn main:
   const msg "Hello, world!"
   print msg
            
   let x 1
   add x 2 2
   print x

   if msg == x:
      print "???"
   else:
      move x 1
   end
end

old:
   mov	edx,len     # message length
   mov	ecx,msg     # message to write
   mov	ebx,1       # file descriptor (stdout)
   mov	eax,4       # system call number (sys_write)
   int	0x80        # call kernel
	
   mov	eax,1       # system call number (sys_exit)
   int	0x80        # call kernel