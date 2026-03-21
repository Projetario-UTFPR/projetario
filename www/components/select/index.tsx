import * as S from "@radix-ui/react-select";
import { SelectContainer } from "./container";
import { SelectContent } from "./content";
import { SelectHeader } from "./header";
import { SelectItem } from "./item";
import { SelectSkeleton } from "./skeleton";
import { SelectTrigger } from "./trigger";
import { SelectViewportGroup } from "./viewport-group";

export default {
  Root: S.Root,
  Header: SelectHeader,
  Trigger: SelectTrigger,
  Content: SelectContent,
  Item: SelectItem,
  Skeleton: SelectSkeleton,
  Container: SelectContainer,
  ViewportGroup: SelectViewportGroup,
  Group: S.Group,
  Viewport: S.Viewport,
  Label: S.Label,
  Separator: S.Separator,
};
