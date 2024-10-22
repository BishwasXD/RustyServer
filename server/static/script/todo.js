fetch('http://127.0.0.1:8477/')
  .then((response) => {
    if (!response.ok) {
      throw new Error('Network response was not OK');
    }
    return response.json();
  })
  .then((data) => {
    console.log('Response Data:', data);
    mapData(data)
    
  })
  .catch((error) => {
    console.error('Fetch Error:', error);
  });
  
function mapData(data){
data.map((d)=>console.log(d.description))

}