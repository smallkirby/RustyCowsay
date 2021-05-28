# Rusty Cowsay
`cowsay` written in Rust.  
You say that Rust is better.  
  

```rusty-cowsay.sh
$ COWPATH=$HOME/Documents/rusty-cowsay/cows ./morasay < ./test.c
 ______________________
/ #include<stdio.h>     \
| #include<stdlib.h>    |
| #include<sys/types.h> |
| #include<unistd.h>    |
|                       |
| int main(int argc){   |
|   if(0) {             |
|     printf("true");   |
|   } else {            |
|     printf("false");  |
|   }                   |
\ }                     /
 ----------------------
      \
       \
         /  ----   ---- \
        (    ===    ===  )
        (    \=/    \=/  )
         \ @  _______ @  /
```

  
# build & install
Below command would generate binary named `rusty-cowsay` and install it in `/usr/bin/` directory. Also, it generate bash-completion script and install it in `/usr/share/bash-completion/completions/`.
```install.sh
git clone https://github.com/smallkirby/RustyCowsay
cd RustyCowsay && bash ./install.sh
```
