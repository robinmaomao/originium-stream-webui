(async () => {
  const results = []
  for (const hash of ['#/hosts', '#/settings', '#/logs', '#/session', '#/local']) {
    location.hash = hash
    await new Promise((resolve) => setTimeout(resolve, 700))
    results.push({ hash, text: document.body.innerText.slice(0, 420) })
  }
  return results
})()
