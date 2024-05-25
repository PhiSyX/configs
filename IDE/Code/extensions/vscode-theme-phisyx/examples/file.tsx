// @ts-nocheck

import exportParDefaut1 from "./import";
export { export1 } from "./import";
import * as nom1 from "./import";
import { export1 } from "./import";
import { export1 as alias1 } from "./import";
import { export2, export3 } from "./import";
import { export4, export5 as alias2 } from "./import";
import exportParDefaut2, { export1 as alias3 } from "./import";
import exportParDefaut3, * as nom2 from "./import";
import "./import";

interface WindowTitleBarProps {
	title: string;
}

export function WindowTitleBar({ title }: WindowTitleBarProps) {
	const minimize_handler = () => appWindow.minimize();
	const maximize_handler = () => appWindow.toggleMaximize();
	const close_handler = () => appWindow.close();

	return (
		<header data-tauri-drag-region className={style["window-title-bar"]}>
			<p>{title}</p>
			<div className={style["window-title-bar--action"]}>
				<WindowTitleBarMinimize onClick={minimize_handler} />
				<WindowTitleBarRestore onClick={maximize_handler} />
				<WindowTitleBarClose onClick={close_handler} />
			</div>
		</header>
	);
}