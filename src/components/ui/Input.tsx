import { clsxm } from "@utils/clsxm";
import { JSX, splitProps } from "solid-js";

export function Input(props: JSX.InputHTMLAttributes<HTMLInputElement>) {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <input
      {...others}
      class={clsxm(
        "w-full rounded-md border border-neutral-300 py-2 px-4",
        local.class
      )}
    />
  );
}
