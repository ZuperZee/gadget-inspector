import { JSX } from "solid-js";

export function Button(props: JSX.ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      class="border-neutrals-medium-75 mt-2 flex w-full items-center rounded border-2 bg-white p-2 outline-none focus-within:border-black"
      {...props}
    />
  );
}
