import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebars: SidebarsConfig = {
  guideSidebar: [{ type: "autogenerated", dirName: "guide" }],
  apiSidebar: [{ type: "autogenerated", dirName: "api" }],
  exampleSidebar: [{ type: "autogenerated", dirName: "example" }],
};

export default sidebars;
