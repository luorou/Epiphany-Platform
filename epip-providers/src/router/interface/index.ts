export interface MetaProps {
	keepAlive?: boolean;
	requiresAuth?: boolean;
	title?: string;
	key?: string;
}

export interface RouteObject {
	children?: RouteObject[];
	element?: React.ReactNode;
	index?: boolean;
	path?: string;
	meta?: MetaProps;
}