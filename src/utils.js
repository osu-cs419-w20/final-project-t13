const displayTime = (secs) => {
  const str = new Date(secs * 1000).toISOString()
  if (secs < 600) {
    return str.substr(15, 4)
  } else if (secs < 3600) {
    return str.substr(14, 5)
  } else if (secs < 36000) {
    return str.substr(12, 7)
  } else {
    return str.substr(11, 8)
  }
}

export {
  displayTime
}
