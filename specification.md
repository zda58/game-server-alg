## Server Game Setup
{
    "height": i32,
    "width": i32,
    "submarines: i32,
    "destroyers": i32,
    "battleships": i32,
    "carriers": i32
}

Height and width are between 6 and 50 inclusive. 

## Player Ship Setup
{
    "submarines": [
        {"horizontal": bool, "x": i32, "y": i32},
        ...
    ],
    "destroyers": [
        {"horizontal": bool, "x": i32, "y": i32},
        ...
    ],
    "battleships": [
        {"horizontal": bool, "x": i32, "y": i32},
        ...
    ],
    "carriers": [
        {"horizontal": bool, "x": i32, "y": i32},
        ...
    ],
}

Horizontal describes the orientation of the ship. The following coordinates
represent either the top or the left coordinate of the ship depending on the 
orientation. Coordinates must place ships within the board with ship lengths 
of 3, 4, 5, and 6 respectively.

## Server Game State
{
    "current_state": String
}

Current state is either "Ongoing", "Win", "Loss", or "Draw". In the latter 3 
cases the game is over.

## Server Shot Request
{
    "shots": i32
}

Shots represents the number of shots the player is to take.

## Player Shots Taken
{
    "shots": [
        {"x": i32, "y": i32},
        ...
    ]
}

Shots a player desires to take. Shots must be of correct length and inside the board.

## Server Shot Report
{
    "shots_hit": [
        {"x": i32, "y": i32},
        ...
    ],
    "coords_damaged": [
        {"x": i32, "y": i32},
        ...
    ]
}

The shots that a player hit, and the coordinates that the other player shot.