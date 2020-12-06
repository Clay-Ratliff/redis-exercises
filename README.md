# Redis Example Exercises

There are two examples of communicating with Redis.
One is written in Rust, the other in .NET Core.

Both examples take 2 optional command line parameters, one will be the url pointing to the DB to be inserted into, the other will be the url of the DV to read the data from.

The .NET core application was created using VS Code, and the directory contains the .csproj file.  As I am on a linux machine the executable was build and run using the dotnet command.
`donet run -- -i <redis-url-for-insert> -r <redis-url-for-reading-from>`

The Rust version was buillt and run using cargo, `cargo run -- -i <redis-url-for-insert> -r <redis-url-for-reading-from>`

For either command, complete argument information is abvailable using, `dotnet run -- --help`, or `cargo run -- --help`.

In either case
1. clone the repo locally
1. install [Rust](https://www.rust-lang.org/tools/install) or [.NET Core](https://dotnet.microsoft.com/download) locally, as appropriate.
1. For the Rust, cd into the root directory (where the Cargo.toml file is located) and run the Rust cargo command given above.  This will build, and execute, the utility.  It will load the data into the given url, and display it in reverse order after reading it from the server server passed as the reading url.  Executing `cargo run -- --help` will provide information about the command arguments.
1. For the .NET Core version, cd into the root directory (where the Program.cs file is located) and execute the dotnet command given above, again passing it the two url arguments.  This will build and execute the utility, isnerting the data at the given insert url, and reading it in reverse order from the url given for the reading DB.

Data is written to the database incrementally from 1 to 100, and the score is the value of the inserted ordinal, _i.e_, value=1 will have a of score=1.  These are then read in reverse order by score, _i.e._, 100 to 1.

These utilities are intended only as a reference on basic communication with Redis.  There are many improvements that can be implemented to demonstrate more complex, and real-world interactions.  Below are just a few that immediately come to mind.
1. Enable TLS for security
1. Use pooled connections
1. Use async connections
1. Use batched insertions rather than inserting one element at a time
