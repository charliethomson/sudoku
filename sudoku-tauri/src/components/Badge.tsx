import { ReactElement } from "react";

export type BadgeProps = {
  StartIcon?: ReactElement;
  label: string;
  EndIcon?: ReactElement;
};

function Badge({ StartIcon, EndIcon, label }: BadgeProps) {
  return (
    <div className="flex items-center gap-2 bg-slate-700 dark:bg-slate-400 dark:text-slate-800 w-fit px-4 py-1 rounded-lg fill-white drop-shadow-lg">
      {StartIcon ?? null}
      <span className="tracking-wider text-lg select-none">{label}</span>
      {EndIcon ?? null}
    </div>
  );
}

export default Badge;
