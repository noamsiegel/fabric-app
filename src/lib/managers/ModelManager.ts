import { invoke } from "@tauri-apps/api/core";
import { timer } from "execution-time-decorators";
import { defaultModelStore, defaultVendorStore } from "$lib/stores/models";


// export interface VendorOption {
//   value: string;
//   label: string;
// }


export class ModelSettingsManager {
  vendors: string[] = [];
  currentVendor: string = '';
  defaultVendor: string = '';
  selectedVendor: string = '';

  @timer()
  async loadVendors() {
    try {
      const vendors = await invoke("get_vendors");
      this.vendors = vendors as string[];
    } catch (err) {
      this.vendors = [];
    }
  }

  @timer()
  async loadDefaultModel() {
    try {
      const model = await invoke("get_secret", { key: "DEFAULT_MODEL" });
      console.log("default model", model);
      defaultModelStore.set(model as string);
    } catch (err) {
      console.error("Failed to load default model:", err);
    }
  }

  @timer()
  async loadDefaultVendor() {
    try {
      const vendor = await invoke("get_secret", { key: "DEFAULT_VENDOR" });
      this.currentVendor = vendor as string;
      this.selectedVendor = vendor as string ;
    } catch (err) {
      console.error("Failed to load default vendor:", err);
    }
  }

  @timer()
  async saveDefaultModel(model: string) {
    try {
      await invoke("update_secret", {
        key: "DEFAULT_MODEL",
        value: model,
      });
      defaultModelStore.set(model);
    } catch (err) {
      console.error("Failed to save default model:", err);
    }
  }

  @timer()
  async saveDefaultVendor(vendor: string) {
    if (vendor === this.currentVendor) return;
    try {
        await invoke("update_secret", {
            key: "DEFAULT_VENDOR",
            value: vendor,
        });
        this.currentVendor = vendor;
        defaultVendorStore.set(vendor);
    } catch (err) {
        console.error("Failed to save default vendor:", err);
    }
}

  @timer()
  async resetDefaults() {
    try {
      await invoke("update_secret", { key: "DEFAULT_MODEL", value: "" });
      await invoke("update_secret", { key: "DEFAULT_VENDOR", value: "" });
      this.currentVendor = "";
      this.defaultVendor = ("");
      this.selectedVendor = "" ;
    } catch (err) {
      console.error("Failed to reset defaults:", err);
    }
  }
}
