This is my first official "fight" with Rust's borrow-checker, so to speak. Now I'm officially a Rust newbie :)

Here are a few things I picked up:
1) Functional flavors: immutability is the default, the beloved Unit Type (), type EVERYTHING and reap the benefits!! Luckily Rust has statements as well which made for a pleasant experience   
2) Result<T, E> and Option<T> are pretty versatile for error-handling and come with a neat set of tools   
   a) pattern-matching and if-lets (sugar for Err(_) => {})   
   b) unwrap() <- will panic on Err(_) or None   
   c) unwrap_or_else(|err| {... handle error here ...})   
   d) map() <- transform Ok(E) into Ok(F)   
   e) map_err() <- transform Err(E) into Err(F)   
   f) .ok_or() <- transposition from Option to Result   
   f) ? <- short-circuit return the error instead of storing it (must be within signature w/ Result return type)   
   g) I really liked that iter.next() returned an Option type, making CLI flag parsing pretty easy   
3) Debug trait was neat. My (uninformed) intuition is that traits are more flexible than classical OOP so
                          I'm excited to experiment more with traits.   
4) I don't understand macros. Why are print! and vec! macros? A question to be fully answered later   
5) Parametric Polymorphism: parse::<T> was very handy. Apparently ::<> is called turbofish   
6) References vs Pointers (Borrowing versus Ownership)   
   a) passing stack values automatically clones   
   b) use .clone() to clone on the heap   
   c) one owner at a time   
   d) one mutable reference at a time   
   e) we're usually passing references since we want to be careful throwing ownership around   
   f) apparently, standard library drop is implemented by: pub fn drop<T>(_x: T) {}
       this takes advantage of Rust's scoping rule. We take in a pointer (not a reference) and
       immediately push it out of scope forcing the value to get dropped. I found this
       implementation pretty funny, for some unknown reason.   
7) Systems-Level Stuff   
   a) Due to not having taking any real systems courses yet (except intro), I had to refresh myself and pick
      up some systems-level knowledge for this project, which was a real plus   
   b) io::stdout().flush().unwrap();   
       flushing the standard out buffer manually while multi-threading to keep up with finding open ports in real-time   
   c) TcpStream::connect((ip_addr, port))   
       synchronously searching for a TCP connection to a socket. I want to figure out how to do this asynchronously later   
   d) let (tx, rx) = channel();   
        for blah in iterator { let tx = tx.clone(); thread.spawn(move || { thread_do_stuff_function })};   
       Neat pattern. tx and rx have types Sender<T> and Receiver<T> respectively. The way the channel works is that the
       two threads have access to shared data. We clone a new sender for every new thread. The thread uses a move closure
       in order to establish dominance (take ownership) and prevent other threads from causing data races. Oh also, don't
       forget to drop(tx) before looking through the goodies in rx.     
9) Atomic Reference Counting (Arc Pointers)   
   a) Following the same thread (XD) as above, I ran into some trouble when using the move closure with multi-threading.
     I tried to pass in a reference &args to each thread, under the mistaken assumption that a new reference would be
     created for each thread. However, the first thread selfishly and greedily took ownership of my reference and left
     all of my other threads impotent :( I thus had to wrap my args struct in an Arc pointer that is specifically
     designed to be thread-safe via immutability, which I had intended anyways but Rust's borrow checker forced me to
     explicitly declare my intentions >:(   
   b) I want to test Box<T> and Rc<T> as well. It also wouldn't hurt to understand C++'s RAII and smart pointers, which
    would probably contextualize Rust's approach to pointers.   
