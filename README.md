# battleshipBot

To build server:
cargo build --manifest-path=server/Cargo.toml

To build player:
cargo build --manifest-path=player/Cargo.toml

to run either:
cargo run --bin server/player

Specifications:
The server hoster will specify the board size and how many of each ship
either player is allowed to have in the following format:

{
    message: "game_information",
    height: i32,
    width: i32,
    ships: {
        SUBMARINES: i32,
        DESTROYERS: i32,
        BATTLESHIPS: i32,
        CARRIERS: i32
    }
}
The players will then generate their boards and ships with a 15 second limit.
Note that the board is indexed such that the y axis is reversed while the x axis
is typical.

The players will respond to the server with a list of their ships
in the following format where each subsection MUST be of length corresponding
to the number of ships with a number of coordinates both consecutive, of correct
ship length, and non-overlapping:
{
    "SUBMARINES": [
        [{"x": x, "y": y}, {"x": x, "y": y}, {"x": x, "y": y}],
        [{"x": x, "y": y}, {"x": x, "y": y}, {"x": x, "y": y}],
        ...
    ],
    "DESTROYERS": [
        ...
    ],
    "BATTLESHIPS": [
        ...
    ],
    "CARRIERS": [
        ...
    ]
}
Afterwards, the server will begin the game. The player will ping a player with the number
of shots that it is allowed to take determined by how many ships of theirs which are still 
alive, and the player will take shots on the opponents board.

The shots will be communicated via the following format:
server:
{
    "message": "shot_count",
    "shots": i32
}
player:
{
    "shots": [
        {
            "x": x,
            "y": y
        },
        {
            "x": x,
            "y": y
        },
        ...
    ]
}
If the player shoots more shots than allowed, then the player will lose automatically.

Once both players have shot, the server will send each player which coords on their board were hit,
and which coords on the opponent's board were hit in the following format. 
{
    "message": "shot_information", 
    "shots_hit": [
        {"x": x, "y": y},
        ...
    ],
    "coords_damaged": [
        {"x": x, "y": y},
        ...
    ]
}
It is up to the player to keep track of this information to make informed decisions.

This loop will continue until a player loses or a draw occurs, which will yield the following
messages to the corresponding players:

Win:
{
    "message": "game_result",
    "state": "win"
}
Lose:
{
    "message": "game_result",
    "state": "lose"
}
Draw:
{
    "message": "game_result",
    "state": "draw"
}