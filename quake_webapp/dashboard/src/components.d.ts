/* eslint-disable */
/* tslint:disable */
/**
 * This is an autogenerated file created by the Stencil compiler.
 * It contains typing information for all components that exist in this project.
 */
import { HTMLStencilElement, JSXBase } from "@stencil/core/internal";
import { ActionDefine } from "./components/quake-dashboard/quake-dashboard";
export namespace Components {
    interface FlowView {
    }
    interface QuakeDashboard {
        "indexName": string;
    }
}
declare global {
    interface HTMLFlowViewElement extends Components.FlowView, HTMLStencilElement {
    }
    var HTMLFlowViewElement: {
        prototype: HTMLFlowViewElement;
        new (): HTMLFlowViewElement;
    };
    interface HTMLQuakeDashboardElement extends Components.QuakeDashboard, HTMLStencilElement {
    }
    var HTMLQuakeDashboardElement: {
        prototype: HTMLQuakeDashboardElement;
        new (): HTMLQuakeDashboardElement;
    };
    interface HTMLElementTagNameMap {
        "flow-view": HTMLFlowViewElement;
        "quake-dashboard": HTMLQuakeDashboardElement;
    }
}
declare namespace LocalJSX {
    interface FlowView {
    }
    interface QuakeDashboard {
        "indexName"?: string;
        "onDispatchAction"?: (event: CustomEvent<ActionDefine>) => void;
    }
    interface IntrinsicElements {
        "flow-view": FlowView;
        "quake-dashboard": QuakeDashboard;
    }
}
export { LocalJSX as JSX };
declare module "@stencil/core" {
    export namespace JSX {
        interface IntrinsicElements {
            "flow-view": LocalJSX.FlowView & JSXBase.HTMLAttributes<HTMLFlowViewElement>;
            "quake-dashboard": LocalJSX.QuakeDashboard & JSXBase.HTMLAttributes<HTMLQuakeDashboardElement>;
        }
    }
}
