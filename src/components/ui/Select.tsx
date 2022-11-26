import clsx from "clsx";
import { For, JSX, Show, splitProps } from "solid-js";

interface SelectItem {
  value: string | number;
  text?: string;
  selected?: boolean;
}

interface Props extends JSX.SelectHTMLAttributes<HTMLSelectElement> {
  labelText?: JSX.Element;
  items: SelectItem[];
}

export function Select(props: Props) {
  const [p, rest] = splitProps(props, ["class", "id", "labelText", "items"]);
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
        <select
          {...rest}
          id={p.id}
          class={clsx(
            "w-full appearance-none bg-white p-1 focus:outline-none",
            p.class
          )}
        >
          <For each={p.items}>
            {(a) => (
              <option value={a.value} selected={a.selected}>
                {a.text ?? a.value}
              </option>
            )}
          </For>
        </select>
      </div>
    </div>
  );
}
