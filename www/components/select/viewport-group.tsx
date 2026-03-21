import type { PropsWithChildren } from "react";
import Select from ".";

type Props = PropsWithChildren<{ className?: string }>;

export function SelectViewportGroup({ children, className }: Props) {
  return (
    <Select.Viewport>
      <Select.Group>
        <Select.Container className={className}>{children}</Select.Container>
      </Select.Group>
    </Select.Viewport>
  );
}
