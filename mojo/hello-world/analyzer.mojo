from std.python import Python, PythonObject

def calculate_average(temps: List[Float64]) raises -> Float64:
    if len(temps) == 0:
        raise Error("No temperature data")

    var total: Float64 = 0.0
    for index in range(len(temps)):
        total += temps[index]
    return total / Float64(len(temps))

def main():
    print("Temperature Analyzer")
    var temps: List[Float64] = [20.5, 22.3, 19.8, 25.1]
    print("Recorded", len(temps), "temperatures")

    for index in range(len(temps)):
        print("  Day {}: {}°C".format(index + 1, temps[index]))

    try:
        var avg = calculate_average(temps)
        print("Average: {}°C".format(avg))

        if avg > 25.0:
            print("Status: Hot week")
        elif avg > 20.0:
            print("Status: Comfortable week")
        else:
            print("Status: Cool week")

        var np = Python.import_module("numpy")
        var pytemps: PythonObject = [20.5, 22.3, 19.8, 25.1]
        print("Temperature standard deviation:", np.std(pytemps))
        _ = pytemps^ # Allow the Python object to deallocate
    except e:
        print("Error:", e)
