(async () => {
  const click = (label) => {
    const button = [...document.querySelectorAll('button')].find((item) =>
      item.textContent.trim().includes(label),
    )
    if (button) {
      button.click()
      return true
    }
    return false
  }
  const steps = []
  for (let index = 0; index < 4; index += 1) {
    steps.push({
      clicked: click('确认'),
      hash: location.hash,
      active: document.querySelector('.step.active')?.textContent,
    })
    await new Promise((resolve) => setTimeout(resolve, 350))
  }
  const finish = click('开始使用')
  await new Promise((resolve) => setTimeout(resolve, 1500))
  return {
    steps,
    finish,
    hash: location.hash,
    drawers: document.querySelectorAll('.v-navigation-drawer').length,
    text: document.body.innerText.slice(0, 900),
  }
})()
