fn main() {
   enum Commands {
    Add { description: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
   }

   match cli.command {
    Commands::Add { description } => commands::add(description),
    Commands::List => commands::list(),
    Commands::Done { id } => commands::done(id),
    Commands::Delete { id } => commands::delete(id),
   }
}
