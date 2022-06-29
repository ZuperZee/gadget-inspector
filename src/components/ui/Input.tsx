import { clsxm } from "@utils/clsxm";
import { JSX, Show, splitProps } from "solid-js";

export function Input(
  props: JSX.InputHTMLAttributes<HTMLInputElement> & { labelText?: JSX.Element }
) {
  const [p, rest] = splitProps(props, ["class", "id", "labelText", "children"]);
  return (
    <div class="w-full">
      {
        <Show when={p.labelText}>
          {
            <label
              class={clsxm("flex w-full flex-col items-start")}
              for={props.id}
            >
              {p.labelText}
            </label>
          }
        </Show>
      }
      <div
        class={clsxm(
          "border-neutrals-medium-75 mt-2 flex w-full items-center rounded border-2 bg-white p-1 focus-within:border-black"
        )}
      >
        <input
          {...rest}
          id={p.id}
          class={clsxm("w-full bg-white p-1 focus:outline-none", p.class)}
        />
        {p.children}
      </div>
    </div>
  );
}
