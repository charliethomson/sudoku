import { useEffect, useState } from "react";
import { getTheme, setTheme } from "../lib/theme";

function ThemeManager() {
  const [interval, _setInterval] = useState<number | null>(null);

  useEffect(() => {
    if (interval) clearInterval(interval);
    else setTheme(getTheme());

    _setInterval(setInterval(() => setTheme(getTheme()), 30_000));
  }, []);

  return <span id="thm" />;
}

export default ThemeManager;
