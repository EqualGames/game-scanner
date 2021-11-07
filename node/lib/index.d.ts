export declare interface Game {
    _type: string;
    id: string;
    name: string;
    path: string | undefined;
    commands: GameCommands;
    state: GameState;
}

export declare interface GameCommands {
    install: Array<string> | undefined;
    launch: Array<string> | undefined;
    uninstall: Array<string> | undefined;
}

export declare interface GameState {
    installed: boolean;
    needs_update: boolean;
    downloading: boolean;
    total_bytes: number | undefined;
    received_bytes: number | undefined;
}


export declare const manager: {
    install_game(game: Game): void;

    launch_game(game: Game): void;

    get_processes(game: Game): Array<number> | undefined;

    close_game(game: Game): void;
}

export declare const amazon: {
    executable(): string | undefined;

    find(id: string): Game | undefined;

    games(): Array<Game>;
}

export declare const blizzard: {
    executable(): string | undefined;

    find(id: string): Game | undefined;

    games(): Array<Game>;
}

export declare const epicgames: {
    executable(): string | undefined;

    find(id: string): Game | undefined;

    games(): Array<Game>;
}

export declare const gog: {
    executable(): string | undefined;

    find(id: string): Game | undefined;

    games(): Array<Game>;
}

export declare const origin: {
    executable(): string | undefined;

    find(id: string): Game | undefined;

    games(): Array<Game>;
}

export declare const riotgames: {
    executable(): string | undefined;

    find(id: string): Game | undefined;

    games(): Array<Game>;
}

export declare const steam: {
    executable(): string | undefined;

    find(id: string): Game | undefined;

    games(): Array<Game>;
}

export declare const ubisoft: {
    executable(): string | undefined;

    find(id: string): Game | undefined;

    games(): Array<Game>;
}
