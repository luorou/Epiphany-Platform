import { RouteObject } from "@/router/interface/index";

export function searchRoute(path: string, routes: RouteObject[] = []): RouteObject {
      let result: RouteObject = {};
      for (let item of routes) {
            if (item.path === path) return item;
            if (item.children) {
                  const res = searchRoute(path, item.children);
                  if (Object.keys(res).length) result = res;
            }
      }
      return result;
}

