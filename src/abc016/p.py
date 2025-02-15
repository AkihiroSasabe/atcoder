import matplotlib.pyplot as plt

def plot_segments_from_input(save_path=None):
    import sys
    input_data = sys.stdin.read()
    lines = input_data.strip().split("\n")
    Ax, Ay, Bx, By = map(float, lines[0].split())
    N = int(lines[1])
    points = [tuple(map(float, line.split())) for line in lines[2:N+2]]
    
    fig, ax = plt.subplots()
    
    # Plot the primary line segment (Ax, Ay) -> (Bx, By)
    ax.plot([Ax, Bx], [Ay, By], 'r-', label="Main Segment")
    
    # Extract X and Y coordinates from points and plot polyline
    Xs, Ys = zip(*points)
    ax.plot(Xs, Ys, 'bo-', label="Polyline")
    
    # Formatting the plot
    ax.set_xlabel("X")
    ax.set_ylabel("Y")
    ax.legend()
    ax.grid(True)
    
    if save_path:
        plt.savefig(save_path)
    else:
        plt.show()

# Read input from standard input and plot
def main():
    plot_segments_from_input(save_path="d_plot.png")

if __name__ == "__main__":
    main()
