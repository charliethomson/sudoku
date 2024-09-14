import { PuzzleSchemaSource } from "../api/types/puzzle";
import UnknownIcon from "mingcute_icon/svg/system/question_line.svg?react";
import NytLogo from "../assets/nyt.svg?react";
import "./MingCuteIcon.css";
import Badge from "./Badge";

export type SourceIconProps = {
  source: PuzzleSchemaSource;
};
function SourceBadge({ source }: SourceIconProps) {
  let Icon = <UnknownIcon className="_mgc" />;

  let publisher = "Unknown";

  switch (source) {
    case "Nyt":
      Icon = <NytLogo className="cursor-default" />;
      publisher = "NYT";
      break;
    default:
  }

  return <Badge StartIcon={Icon} label={publisher} />;
}

export default SourceBadge;
