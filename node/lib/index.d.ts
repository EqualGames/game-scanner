export declare interface Game {
    _type: string;
    id: string;
    name: string;
    path: string;
    commands: GameCommands;
    state: GameState;
}

export declare interface GameCommands {
    install?: Array<string>;
    launch: Array<string>;
    uninstall?: Array<string>;
}

export declare interface GameState {
    installed: boolean;
    needs_update: boolean;
    downloading: boolean;
    total_bytes?: number;
    received_bytes?: number;
}

export declare namespace manager {
    function launch_game(game: Game): void;

    function close_game(game: Game): void;
}

export declare namespace amazon {
    function games(): Array<Game>;
}

export declare namespace blizzard {
    function games(): Array<Game>;
}

export declare namespace epicgames {
    function games(): Array<Game>;
}

export declare namespace gog {
    function games(): Array<Game>;
}

export declare namespace origin {
    function games(): Array<Game>;
}

export declare namespace riotgames {
    function games(): Array<Game>;
}

export declare namespace steam {
    function find(id: string): Game | undefined;

    function games(): Array<Game>;
}

export declare namespace ubisoft {
    function games(): Array<Game>;
}
