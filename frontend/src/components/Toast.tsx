
export default function Toast(props: {
  message: string;
  isSuccess: boolean;
  onClose: () => void;
}) {
  return (
    <div
      class={`toast show position-fixed bottom-0 end-0 m-3 ${props.isSuccess ? "bg-success text-white" : "bg-danger text-white"
        }`}
      role="alert"
    >
      <div class="toast-body">
        {props.message}
        <button
          type="button"
          class="btn-close btn-close-white ms-2 me-2"
          aria-label="Close"
          onClick={props.onClose}
        ></button>
      </div>
    </div>
  );
}
