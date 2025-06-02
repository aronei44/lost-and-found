import { Modal, ModalBody, ModalFooter, ModalHeader } from 'flowbite-react';

const ModalComponent = ({ show, onClose, children, title }: { show: boolean; onClose: () => void; children: React.ReactNode, title: string }) => {
    return (
        <Modal show={show} onClose={onClose} size='7xl'>
            <ModalHeader>
                {title}
            </ModalHeader>
            <ModalBody>
                {children}
            </ModalBody>
            <ModalFooter>
                <button className="btn btn-primary" onClick={onClose}>
                    Close
                </button>
            </ModalFooter>
        </Modal>
    );
}

export default ModalComponent;