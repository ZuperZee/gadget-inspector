import { clsxm } from "@utils/clsxm";
import { Index, JSX, splitProps } from "solid-js";

export interface ModbusBitData {
  addresses: number[];

  bool: boolean[];
}

function HeaderCell(props: JSX.ThHTMLAttributes<HTMLTableCellElement>) {
  const [p, rest] = splitProps(props, ["class"]);
  return (
    <th {...rest} class={clsxm("sticky top-0 z-20 bg-inherit p-2", p.class)}>
      {props.children}
    </th>
  );
}

function DataCell(props: JSX.TdHTMLAttributes<HTMLTableCellElement>) {
  const [p, rest] = splitProps(props, ["class"]);
  return (
    <td {...rest} class={clsxm("px-2", p.class)}>
      {props.children}
    </td>
  );
}

export function ModbusBitTable(props: { modbusData?: ModbusBitData }) {
  return (
    <div class="grow overflow-x-auto shadow-sm">
      <table class="w-full text-left">
        <thead>
          <tr class="bg-neutral-100">
            <HeaderCell class="left-0 z-30">Addr</HeaderCell>

            <HeaderCell>Bool</HeaderCell>
          </tr>
        </thead>
        <tbody class="font-mono">
          <Index each={props.modbusData?.addresses}>
            {(_, i) => (
              <tr class="whitespace-nowrap odd:bg-white even:bg-neutral-100 [&:nth-child(4n+1)]:bg-neutral-200">
                <DataCell class="sticky left-0 z-10 bg-inherit">
                  {props.modbusData?.addresses[i]}
                </DataCell>

                <DataCell class="w-full">
                  {props.modbusData?.bool[i] ? "True" : "False"}
                </DataCell>
              </tr>
            )}
          </Index>
        </tbody>
      </table>
    </div>
  );
}
