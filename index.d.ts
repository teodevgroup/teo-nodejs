/*~ Teo app. */
declare class App {

  /*~ Create a Teo app. */
  constructor()

  /*~ Load the main schema file. */
  load(schemaFileName?: string): void

  /*~ Run the app, this normally starts the server. */
  run(): Promise<void>
}
