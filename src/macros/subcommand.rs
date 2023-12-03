
#[macro_export]
macro_rules! handle_subcommand {
    ($cli:expr, $subcommand:path) => {
        use scripts::{$subcommand};
        if let Some(subcli) = $cli.subcommand_matches(stringify!($subcommand)) {
            use $subcommand as sub;
            sub::matchd(subcli);
        }
    };
}

#[macro_export]
macro_rules! add_subcommand {
    ($cli:expr, $subcommand:path) => {{
        use scripts::{$subcommand};
        use $subcommand as sub;
        $cli.subcommand(sub::command())
    }};
}

#[macro_export]
macro_rules! add_subcommands {
    ($cli:expr, $($subcommand:path),*) => {
        {
            let mut cli = $cli;
            $(
                cli = add_subcommand!(cli, $subcommand);
            )*
            cli
        }
    };
}

#[macro_export]
macro_rules! handle_subcommands {
    ($cli:expr, $($subcommand:path),*) => {
        {
            $(
                handle_subcommand!($cli, $subcommand);
            )*
        }
    };
}

#[macro_export]
macro_rules! subcommands {
    ($cli: expr, $($arg:path),*) => {{
        let ncli = add_subcommands![$cli, $($arg),*];
        let ncli = ncli.get_matches();
        handle_subcommands![ncli, $($arg),*];
        ncli
    }};
}