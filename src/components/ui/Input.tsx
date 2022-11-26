import clsx from "clsx";
import { JSX, Show, splitProps } from "solid-js";

export function Input(
  props: JSX.InputHTMLAttributes<HTMLInputElement> & { labelText?: JSX.Element }
) {
  const [p, rest] = splitProps(props, ["class", "id", "labelText"]);
  return (
    <div class="mt-2 w-full">
      {
        <Show when={p.labelText}>
          {
            <label class={"flex w-full flex-col items-start"} for={p.id}>
              {p.labelText}
            </label>
          }
        </Show>
      }
      <div
        class={
          "border-neutrals-medium-75 mt-1 flex w-full items-center rounded border-2 bg-white p-1 focus-within:border-black"
        }
      >
        <input
          {...rest}
          id={p.id}
          class={clsx("w-full bg-white p-1 focus:outline-none", p.class)}
        />
      </div>
    </div>
  );
}
