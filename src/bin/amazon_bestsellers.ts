export class ProduceCategories extends ChainLogicBase<CategoryTask, string> {
  async *parseData({ data, task, message }: { data: string; task: CountryTask; message: ConsumeMessage }) {
    const dom = new DOMParser(data);
    
    // Try multiple possible selectors
    let categories = dom.select('div[role="treeitem"] a').tryMultiple() || 
                    dom.select('.zg-browse-item a').tryMultiple() ||
                    dom.select('.zg-browse-root a').tryMultiple();
    
    if (!categories || categories.length === 0) {
      // Check for anti-automation
      if (dom.root.html().includes("To discuss automated access to Amazon data")) {
        console.log("[Amazon DE] Anti-automation detected, retrying with different method");
        // Retry with different fetch method
        const retryData = await this.requestResource({
          url: task.countryLink,
          fetchMethod: "cloudflare_worker_new",
          proxyOptions: { 
            country: task.locale,
            zone: "residential" 
          }
        });
        return this.parseData({ data: retryData, task, message });
      }
      throw new Error("No category elements found on page");
    }

    // Rest of the parsing logic
    for (const category of categories) {
      // ...existing code...
    }
  }
}