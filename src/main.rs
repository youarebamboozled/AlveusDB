mod logger;
mod listener;
mod utils;
mod http_response;
mod http_header;
mod http_handler;
mod json;

use crate::logger::log_level::LogLevel;

fn main() {
    println!("

               AAA              lllllll                                                                        DDDDDDDDDDDDD     BBBBBBBBBBBBBBBBB
              A:::A             l:::::l                                                                        D::::::::::::DDD  B::::::::::::::::B
             A:::::A            l:::::l                                                                        D:::::::::::::::DDB::::::BBBBBB:::::B
            A:::::::A           l:::::l                                                                        DDD:::::DDDDD:::::BB:::::B     B:::::B
           A:::::::::A           l::::vvvvvvv           vvvvvvveeeeeeeeeeee   uuuuuu    uuuuuu     ssssssssss    D:::::D    D:::::DB::::B     B:::::B
          A:::::A:::::A          l::::lv:::::v         v:::::ee::::::::::::ee u::::u    u::::u   ss::::::::::s   D:::::D     D:::::B::::B     B:::::B
         A:::::A A:::::A         l::::l v:::::v       v:::::e::::::eeeee:::::eu::::u    u::::u ss:::::::::::::s  D:::::D     D:::::B::::BBBBBB:::::B
        A:::::A   A:::::A        l::::l  v:::::v     v:::::e::::::e     e:::::u::::u    u::::u s::::::ssss:::::s D:::::D     D:::::B:::::::::::::BB
       A:::::A     A:::::A       l::::l   v:::::v   v:::::ve:::::::eeeee::::::u::::u    u::::u  s:::::s  ssssss  D:::::D     D:::::B::::BBBBBB:::::B
      A:::::AAAAAAAAA:::::A      l::::l    v:::::v v:::::v e:::::::::::::::::eu::::u    u::::u    s::::::s       D:::::D     D:::::B::::B     B:::::B
     A:::::::::::::::::::::A     l::::l     v:::::v:::::v  e::::::eeeeeeeeeee u::::u    u::::u       s::::::s    D:::::D     D:::::B::::B     B:::::B
    A:::::AAAAAAAAAAAAA:::::A    l::::l      v:::::::::v   e:::::::e          u:::::uuuu:::::u ssssss   s:::::s  D:::::D    D:::::DB::::B     B:::::B
   A:::::A             A:::::A  l::::::l      v:::::::v    e::::::::e         u:::::::::::::::us:::::ssss::::::DDD:::::DDDDD:::::BB:::::BBBBBB::::::B
  A:::::A               A:::::A l::::::l       v:::::v      e::::::::eeeeeeee  u:::::::::::::::s::::::::::::::sD:::::::::::::::DDB:::::::::::::::::B
 A:::::A                 A:::::Al::::::l        v:::v        ee:::::::::::::e   uu::::::::uu:::us:::::::::::ss D::::::::::::DDD  B::::::::::::::::B
AAAAAAA                   AAAAAAllllllll         vvv           eeeeeeeeeeeeee     uuuuuuuu  uuuu sssssssssss   DDDDDDDDDDDDD     BBBBBBBBBBBBBBBBB






                                                                                                                                                     ");

    logger::Builder::new()
        .level(LogLevel::Debug)
        .build();
    debug!("Logger initialized");

    let listener = listener::Listener::new();
    listener.listen();
}
