export interface Game {
  _type: string;
  id: string;
  name: string;
  path: string;
  launch_command: string;
}

export declare function games(): Array<Game>;
