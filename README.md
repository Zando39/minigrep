# minigrep
A primitive version of the grep command line tool,
made in rust as a practice project. I followed the rust tutorial
at https://doc.rust-lang.org/book/ch12-00-an-io-project.html
as a starting point, but have also added a few unique features.

BASIC USAGE:

minigrep needs two arguments: a string to search for, and a text file to search in.

eg.

"minigrep to poem.txt"

if you want your search to be case sensitive, you can add a -c at the end, like this:

"minigrep to poem.txt -c"
