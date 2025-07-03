class FirstPageProbe extends ChainLogicBase<EchoParkPaginationTask, string, {}, {}> {
  // ... (getData method)

  async *parseData({ data: html }: { data: any }) {
    // ... (parsing logic)

    for (let i = 1; i * PER_PAGE < validatedData.vehicleSearch.srpVehiclesData.resultCount; i++) {
      yield { page: i };
      // --- ADD DELAY HERE using the standard pattern ---
      const delayMilliseconds = Math.random() * 2000 + 1000; // Example: random delay between 1000ms (1 sec) and 3000ms (3 sec)
      await new Promise(resolve => setTimeout(resolve, delayMilliseconds));
      // ------------------------------------------------
    }
    // ... (code processing cars on the first page)
  }
}