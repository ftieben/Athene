  
  // returns id, date, title for my tasks
  export async function getTaskList() {
    const api = "http://localhost:8080/"

    const res = await fetch(api).catch(function() {console.log("could not connect to " + api)})
    console.log(res)
    if (res) {
      const data = await res.json()

      if (!data) {
        return {
          notFound: true,
        }
      }
      console.log(data)
      return data
    }

    let id = ''
    let date= '2020-01-02'
    let title = 'Could not load Task Data'
    return [
      {id: id, 
      date: date,
      title: title}
    ]
  }


    // returns id, date, title WIP
    export async function getTaskData(id) {
      const res = await fetch("http://localhost:8080/", id).catch(function() {console.log("could connect to thingi " )})

      if (res) {
        const data = await res.json()
  
        if (!data) {
          return {
            notFound: true,
          }
        }
        console.log(data)
      }

      let date= '2020-01-02'
      let title = 'When to Use Static Generation v.s. Server-side Rendering'
      return {id, date, title}
    }