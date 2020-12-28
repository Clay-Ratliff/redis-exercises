using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using System.Linq;
using ServiceStack.Redis;

namespace dot_net_example
{
    public class InsertAndReadOrdinals
    {   
        static void Main(string[] args)
        {
            var rootCommand = new RootCommand {
                new Option<string?>(
                    new[] { "--insert-url", "-i" },
                    description: "URL for the DB to insert the ordinals into"),
                new Option<string?>(
                    new[] { "--read-url", "-r" },
                    description: "URL for the DB to read the ordinals from")

            };

            rootCommand.Description = "C# .NET Core simple example of connecting to a Redis DB";
            rootCommand.Handler = CommandHandler.Create<string?, string?>((insertUrl, readUrl) => {
                Console.WriteLine($"The value for --insert-url is: {insertUrl ?? "Not provided, using defaults"}");
                Console.WriteLine($"The value for --read-url is: {readUrl ?? "Not provided, using defaults"}");
                var programInstance = new InsertAndReadOrdinals();
                var writeClient = new RedisClient(insertUrl ??  "redis://localhost:12000" );
                var readClient = new RedisClient(readUrl ??  "redis://localhost:12000");
                programInstance.populateRedisWithOrdinals(writeClient);
                programInstance.readOrdinalsFromRedis(readClient);
            });

            rootCommand.Invoke(args);

        }

        public void populateRedisWithOrdinals(RedisClient client) {
            foreach (int number in Enumerable.Range(1, 100)) {
                client.AddItemToSortedSet("dot-net-ordinals", $"{number}", number);
            }
        }

        public void readOrdinalsFromRedis(RedisClient client) {
            Console.WriteLine("Reading from redis");
            var values = client.GetAllItemsFromSortedSetDesc("dot-net-ordinals");
            foreach (var element in values) {
                Console.WriteLine($"Read {element} from Redis");
            }
        }
    }
}
