import type { Accessor } from "solid-js";
import logo from "../../../assets/logo.svg";
import { Button } from "../../../ui/components/Button";
import { TextInput } from "../../../ui/components/TextInput";
import { Typography } from "../../../ui/components/Typography";

export type WorkspacePanelStrings = {
  welcomeTitle: string;
  learnMore: string;
  greet: string;
  enterName: string;
};

export type WorkspacePanelViewProps = {
  strings: WorkspacePanelStrings;
  name: Accessor<string>;
  onNameChange: (value: string) => void;
  greetMsg: Accessor<string>;
  onGreet: () => void;
};

export const WorkspacePanelView = (props: WorkspacePanelViewProps) => {
  return (
    <main class="container">
      <Typography variant="h1" as="h1">
        {props.strings.welcomeTitle}
      </Typography>

      <div class="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} class="logo solid" alt="Solid logo" />
        </a>
      </div>
      <Typography variant="body" as="p">
        {props.strings.learnMore}
      </Typography>

      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          props.onGreet();
        }}
      >
        <TextInput
          id="greet-input"
          value={props.name()}
          placeholder={props.strings.enterName}
          onChange={props.onNameChange}
        />
        <Button type="submit">{props.strings.greet}</Button>
      </form>
      <Typography variant="comment" as="p">
        {props.greetMsg()}
      </Typography>
    </main>
  );
};
