export interface Game {
    _type: string;
    id: string;
    name: string;
    path: string;
    launch_command: string;
}

export declare function games(): Array<Game>;
export declare function launch_game(game: Game): void;
export declare function close_game(game: Game): void;
