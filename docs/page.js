var hash = document.location.hash;
seedbox.value = parseInt(hash.replace("#",""));
seedbox.addEventListener('input', collectSeed);
oldrecipes.addEventListener('change', collectSeed);
probability.addEventListener('change', collectSeed);

//seed handler
function collectSeed() {
  var seed = seedbox.value;
  if (seed.length > 0 && seed <= SEEDMAX && seed > 0) {
    seed = parseInt(seed);
    var tmp = main(seed);
    var mats = tmp[0];
    var prob = tmp[1];
    wseed.innerHTML = seed;
    lc1.innerHTML = mats[0];
    lc2.innerHTML = mats[1];
    lc3.innerHTML = mats[2];
    ap1.innerHTML = mats[3];
    ap2.innerHTML = mats[4];
    ap3.innerHTML = mats[5];
    var all = document.getElementsByTagName("td");
    for (var i = 0; i < all.length; i++) {
      all[i].style.color = '#262626';
    }
    maxalert.innerHTML = "";
  } else {
    wseed.innerHTML = "";
    lc1.innerHTML = "LC#1";
    lc2.innerHTML = "LC#2";
    lc3.innerHTML = "LC#3";
    ap1.innerHTML = "AP#1";
    ap2.innerHTML = "AP#2";
    ap3.innerHTML = "AP#3";
    problc.innerHTML = "";
    probap.innerHTML = "";
    var all = document.getElementsByTagName("td");
    for (var i = 0; i < all.length; i++) {
      all[i].style.color = '#737480';
    }
  }
  if (seed > SEEDMAX) {
    maxalert.innerHTML = "WARNING: Seeds only go up to 4294967295!";
  } else if (seed < 0) {
    maxalert.innerHTML = "Seeds can't be negative!";
  } else {
    maxalert.innerHTML = "";
  }
  if (probability.checked == false) {
    lcheader.innerHTML = "";
    apheader.innerHTML = "";
    problc.innerHTML = "";
    probap.innerHTML = "";
  } else {
    lcheader.innerHTML = "LC RCPU:";
    apheader.innerHTML = "AP RCPU:";
    problc.innerHTML = prob[0] + "%";
    probap.innerHTML = prob[1] + "%";
  }
}

//accordion for advanced info
var acc = document.getElementsByClassName("accordion");
var i;

for (i = 0; i < acc.length; i++) {
  acc[i].addEventListener("click", function() {
    this.classList.toggle("active");

    var panel = this.nextElementSibling;
    if (panel.style.display === "block") {
      panel.style.display = "none";
    } else {
      panel.style.display = "block";
    }
  });
}