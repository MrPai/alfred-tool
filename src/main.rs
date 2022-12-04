mod command;
mod config;
mod error;
mod task;
mod types;

fn main() {
    command::run();
}

// cat << EOB
// {"items": [

// 	{
// 		"title": "hko",
// 		"subtitle": "parallel-heiko",
// 		"arg": "parallel-heiko",
// 		"autocomplete": "hko",
// 	}

// ]}
// EOB
